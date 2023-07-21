#include "RostersController.h"
#include "../GameStateStore.h"

#include <vector>
#include <string>
#include <nlohmann/json.hpp>

using namespace derby_stats;
using namespace derby_stats::api;

using json = nlohmann::json;

typedef struct
{
	string name;
	string number;
} roster_skater;

RostersController::RostersController(const shared_ptr<GameStateStore>& state_store)
{
	this->state_store = state_store;
}

vector<handler_definition> RostersController::get_handlers()
{
	return
	{
		{ http_verb::get, "/api/rosters", [this] { return this->get_rosters(); } },
	};
}

json map_skaters(const map<string, skater>& skaters)
{
	vector<json> result;

	for(const auto& [name, number] : skaters | views::values)
	{
		result.push_back({ name, number });
	}

	return result;
}

string RostersController::get_rosters() const
{
	const auto [home_team, away_team] = this->state_store->get_state();

	const json data = {
		{ "home_team", {
			{ "name", home_team.name },
			{ "skaters", map_skaters(home_team.skaters) }
		}},
		{ "away_team", {
			{ "name", away_team.name },
			{ "skaters", map_skaters(away_team.skaters) }
		}}
	};

	return data.dump();
}
