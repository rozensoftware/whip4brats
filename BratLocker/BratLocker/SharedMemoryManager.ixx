module;

#include "framework.h"
#include <string>

export module SharedMemoryManager;

namespace rozen::sharedmemory
{
	export class SharedMemoryManager
	{
	public:

		SharedMemoryManager() = default;
		SharedMemoryManager(const SharedMemoryManager&) = delete;
		SharedMemoryManager(SharedMemoryManager&&) = delete;
		virtual ~SharedMemoryManager()
		{
			close();
		}

		SharedMemoryManager& operator=(const SharedMemoryManager&) = delete;
		SharedMemoryManager& operator=(SharedMemoryManager&&) = delete;

		auto create(const std::wstring& name, size_t size) noexcept
		{
			m_hMapFile = CreateFileMappingW(INVALID_HANDLE_VALUE, nullptr, PAGE_READWRITE, 0, size, name.c_str());

			if (m_hMapFile == nullptr)
			{
				return false;
			}

			m_pBuf = MapViewOfFile(m_hMapFile, FILE_MAP_ALL_ACCESS, 0, 0, size);

			if (m_pBuf == nullptr)
			{
				CloseHandle(m_hMapFile);
				return false;
			}

			this->size = size;
			return true;
		}

		auto open(const std::wstring& name, size_t size) noexcept
		{
			m_hMapFile = OpenFileMappingW(FILE_MAP_ALL_ACCESS, FALSE, name.c_str());

			if (m_hMapFile == nullptr)
			{
				return false;
			}

			m_pBuf = MapViewOfFile(m_hMapFile, FILE_MAP_ALL_ACCESS, 0, 0, size);

			if (m_pBuf == nullptr)
			{
				CloseHandle(m_hMapFile);
				return false;
			}

			this->size = size;
			return true;
		}

		auto lock() noexcept
		{
			m_hMutex = OpenMutex(
				MUTEX_ALL_ACCESS,
				FALSE,
				L"BratSharedMemoryMutex");

			// Acquire mutex to access shared memory
			return WaitForSingleObject(m_hMutex, INFINITE);
		}

		auto release() noexcept
		{
			// Release mutex
			return ReleaseMutex(m_hMutex);
		}

		void clear() noexcept
		{
			FillMemory(m_pBuf, size - 1, 0);
		}

		void write(const std::wstring& str) noexcept
		{
			if (m_pBuf != nullptr)
			{
				CopyMemory((PVOID)m_pBuf, str.c_str(), str.size() * sizeof(wchar_t));
			}
		}

		void close() noexcept
		{
			if (m_pBuf != nullptr)
			{
				UnmapViewOfFile(m_pBuf);
				m_pBuf = nullptr;
			}

			if (m_hMapFile != nullptr)
			{
				CloseHandle(m_hMapFile);
				m_hMapFile = nullptr;
			}

			if (m_hMutex != nullptr)
			{
				CloseHandle(m_hMutex);
				m_hMutex = nullptr;
			}
		}

		auto getBuffer() const noexcept
		{
			return m_pBuf;
		}

	protected:

			HANDLE m_hMapFile = nullptr;
			HANDLE m_hMutex = nullptr;
			LPVOID m_pBuf = nullptr;
			size_t size;
	};
}