#pragma once

#include <iostream>
#include <string>
#include <format>

namespace derby_stats
{
	enum LOG_LEVEL
	{
		LOG_LEVEL_FATAL,
		LOG_LEVEL_ERROR,
		LOG_LEVEL_WARN,
		LOG_LEVEL_INFO,
		LOG_LEVEL_DEBUG,
		LOG_LEVEL_TRACE
	};

	class Logger
	{
	private:
		inline static LOG_LEVEL log_level = LOG_LEVEL_WARN;

	public:
		static void set_log_level(LOG_LEVEL log_level)
		{
			Logger::log_level = log_level;
		}

		static LOG_LEVEL get_log_level()
		{
			return log_level;
		}

		template<class... Types>
		static void log_fatal(const std::string& format, Types... args)
		{
			log(LOG_LEVEL_FATAL, format, args...);
		}

		template<class... Types>
		static void log_error(const std::string& format, Types... args)
		{
			log(LOG_LEVEL_ERROR, format, args...);
		}

		template<class... Types>
		static void log_warn(const std::string& format, Types... args)
		{
			log(LOG_LEVEL_WARN, format, args...);
		}

		template<class... Types>
		static void log_info(const std::string& format, Types... args)
		{
			log(LOG_LEVEL_INFO, format, args...);
		}

		template<class... Types>
		static void log_debug(const std::string& format, Types... args)
		{
			log(LOG_LEVEL_DEBUG, format, args...);
		}

		template<class... Types>
		static void log_trace(const std::string& format, Types... args)
		{
			log(LOG_LEVEL_TRACE, format, args...);
		}

	private:
		template<class... Types>
		static void log(const LOG_LEVEL log_level, const string_view message, Types&&... args)
		{
			if (Logger::log_level < log_level) return;

			auto const formatted_message = std::vformat(message, std::make_format_args(args...));

			std::cout << formatted_message << endl;
		}
	};
}