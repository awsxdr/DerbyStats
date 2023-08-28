#pragma once

#include "../ScoreboardConnector.h"

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <ranges>
#include <regex>
#include <string>
#include <vector>

namespace derby_stats::state_stores
{
	using namespace std;

	typedef struct
	{
		const string& key;
		const string&& value;
	} update_data;

	template<class TState>
	class StateStore
	{
	public:
		typedef function<void(TState& state, update_data&& data)> state_mapper;
		typedef function<void(TState state)> update_listener_callback;

	private:
		TState state;
		map<string, state_mapper> mappers;
		vector<update_listener_callback> update_listeners;

		void notify_update_listeners()
		{
			for(auto& listener: this->update_listeners)
			{
				listener(this->state);
			}
		}

		void handle_state_update(string key, string value)
		{
			auto const matching_key = get_matching_key(key);

			if (!matching_key.has_value())
				return;

			auto const mapper = this->mappers[matching_key.value()];
			mapper(this->state, { key, move(value) });
		}

		static string escape_regex(const string& value)
		{
			static const auto regex_chars = "().";

			auto value_copy = value;

			unsigned long long find_index = 0;
			while ((find_index = value_copy.find_first_of(regex_chars, find_index)) != string::npos)
			{
				value_copy = value_copy.substr(0, find_index) + "\\" + value_copy.substr(find_index);
				find_index += 2;
			}

			return value_copy;
		}

		optional<string> get_matching_key(const string& target_key)
		{
			static const auto is_wildcard_key = [](const string& key) -> bool
			{
				return key.find("(*)") != string::npos;
			};

			for (const auto& candidate_key : mappers | views::keys)
			{
				if (is_wildcard_key(candidate_key))
				{
					auto regex_string = escape_regex(candidate_key);
					do
					{
						auto const first_wildcard_offset = regex_string.find("\\(*\\)");
						regex_string = regex_string.substr(0, first_wildcard_offset) + R"(\([^\)]+\))" + regex_string.substr(first_wildcard_offset + 5);
					} while (is_wildcard_key(regex_string));

					if (regex_search(target_key, regex(regex_string)))
						return candidate_key;
				}
				else
				{
					if (candidate_key == target_key)
						return candidate_key;
				}
			}

			return {};
		}

	public:
		StateStore(const shared_ptr<ScoreboardConnector>& scoreboard_connector, map<string, state_mapper> mappers)
		{
			scoreboard_connector->set_state_update_handler([this](string key, string value) {
				this->handle_state_update(
					std::move(key),
					std::move(value));
				});

			this->mappers = mappers;

			for (const auto& key : this->mappers | views::keys)
			{
				scoreboard_connector->register_topic(key);
			}
		}

		TState get_state()
		{
			return this->state;
		}

		void add_update_listener(update_listener_callback callback)
		{
			this->update_listeners.push_back(callback);
		}
	};
}