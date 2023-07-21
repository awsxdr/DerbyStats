#include "GameStateStore.h"
#include "Logger.hpp"

#include <functional>
#include <optional>
#include <ranges>
#include <regex>
#include <string>
#include <utility>
#include <nlohmann/json.hpp>

using namespace std;
using namespace derby_stats;

using json = nlohmann::json;

typedef struct
{
	const string& key;
	const string&& value;
} update_data;

typedef function<void(game_state&, update_data&&)> state_mapper;
typedef function<team& (game_state&)> team_state_getter;
typedef function<void(skater&, const string&)> skater_state_setter;

optional<string> get_matching_key(const string& target_key);
string escape_regex(const string& value);

state_mapper map_skater(const team_state_getter& team_getter, const skater_state_setter& skater_setter)
{
	static const auto skater_id_regex = regex(R"(ScoreBoard\.CurrentGame\.Team\(\d\)\.Skater\(([^\)]+)\))");

	return [team_getter, skater_setter](auto&& state, auto&& data)
	{
		smatch matches;
		regex_search(data.key, matches, skater_id_regex);

		if (matches.size() != 2)
			return;

		auto const id = matches[1].str();

		team& team = team_getter(state);

		if (!team.skaters.contains(id))
			team.skaters[id] = {};

		const string value = data.value;
		skater_setter(team.skaters[id], value);
	};
}

void set_skater_name(skater& skater, const string& name)
{
	skater.name = name;
}

void set_skater_number(skater& skater, const string& number)
{
	skater.number = number;
}

const team_state_getter home_team_selector = [](auto&& state) -> team& { return state.home_team; };
const team_state_getter away_team_selector = [](auto&& state) -> team& { return state.away_team; };

map<string, state_mapper> mappers = {
	{ "ScoreBoard.CurrentGame.Team(1).Name", [](auto&& state, auto&& data) { state.home_team.name = data.value; }},
	{ "ScoreBoard.CurrentGame.Team(2).Name", [](auto&& state, auto&& data) { state.away_team.name = data.value; }},
	{ "ScoreBoard.CurrentGame.Team(1).Score", [](auto&& state, auto&& data) { state.home_team.score = stoi(data.value); }},
	{ "ScoreBoard.CurrentGame.Team(2).Score", [](auto&& state, auto&& data) { state.away_team.score = stoi(data.value); }},
	{ "ScoreBoard.CurrentGame.Team(1).Skater(*).Name", map_skater(home_team_selector, set_skater_name)},
	{ "ScoreBoard.CurrentGame.Team(2).Skater(*).Name", map_skater(away_team_selector, set_skater_name)},
	{ "ScoreBoard.CurrentGame.Team(1).Skater(*).RosterNumber", map_skater(home_team_selector, set_skater_number)},
	{ "ScoreBoard.CurrentGame.Team(2).Skater(*).RosterNumber", map_skater(away_team_selector, set_skater_number)},
};

GameStateStore::GameStateStore(const unique_ptr<ScoreboardConnector>& scoreboard_connector)
{
	scoreboard_connector->set_state_update_handler([this](string key, string value) {
		this->handle_state_update(
			std::move(key),
			std::move(value));
	});

	for(const auto& key : mappers | views::keys)
	{
		scoreboard_connector->register_topic(key);
	}
}

void GameStateStore::handle_state_update(const string key, string value)
{
	auto const matching_key = get_matching_key(key);

	if (!matching_key.has_value())
		return;

	auto const mapper = mappers[matching_key.value()];
	mapper(this->state, { key, move(value) });
}

game_state GameStateStore::get_state()
{
	return this->state;
}

string escape_regex(const string& value)
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

