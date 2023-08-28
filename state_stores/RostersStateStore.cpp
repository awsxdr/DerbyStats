#include "RostersStateStore.h"

#include <functional>
#include <regex>
#include <string>

using namespace derby_stats;
using namespace state_stores;

using namespace std;

typedef function<void(roster_skater&, const string&)> skater_state_setter;
typedef function<roster_team& (roster_state&)> team_state_getter;

const team_state_getter home_team_selector = [](auto&& state) -> roster_team& { return state.home_team; };
const team_state_getter away_team_selector = [](auto&& state) -> roster_team& { return state.away_team; };

RostersStateStore::state_mapper map_skater(const team_state_getter& team_getter, const skater_state_setter& skater_setter);

void set_skater_name(roster_skater& skater, const string& name);
void set_skater_number(roster_skater& skater, const string& number);

map<string, RostersStateStore::state_mapper> mappers =
{
	{ "ScoreBoard.CurrentGame.Team(1).Skater(*).Name", map_skater(home_team_selector, set_skater_name)},
	{ "ScoreBoard.CurrentGame.Team(2).Skater(*).Name", map_skater(away_team_selector, set_skater_name)},
	{ "ScoreBoard.CurrentGame.Team(1).Skater(*).RosterNumber", map_skater(home_team_selector, set_skater_number)},
	{ "ScoreBoard.CurrentGame.Team(2).Skater(*).RosterNumber", map_skater(away_team_selector, set_skater_number)},
};

RostersStateStore::RostersStateStore(const shared_ptr<ScoreboardConnector>& scoreboard_connector)
	: StateStore<roster_state>(scoreboard_connector, ::mappers)
{
}

RostersStateStore::state_mapper map_skater(const team_state_getter& team_getter, const skater_state_setter& skater_setter)
{
	static const auto skater_id_regex = regex(R"(ScoreBoard\.CurrentGame\.Team\(\d\)\.Skater\(([^\)]+)\))");

	return [team_getter, skater_setter](auto&& state, auto&& data)
	{
		smatch matches;
		regex_search(data.key, matches, skater_id_regex);

		if (matches.size() != 2)
			return;

		auto const id = matches[1].str();

		roster_team& team = team_getter(state);

		if (!team.skaters.contains(id))
			team.skaters[id] = {};

		const string value = data.value;
		skater_setter(team.skaters[id], value);
	};
}

void set_skater_name(roster_skater& skater, const string& name)
{
	skater.name = name;
}

void set_skater_number(roster_skater& skater, const string& number)
{
	skater.number = number;
}
