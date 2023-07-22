#include "GameStateStore.h"
#include "../ScoreboardConnector.h"

#include <functional>
#include <optional>
#include <ranges>
#include <regex>
#include <string>
#include <utility>
#include <nlohmann/json.hpp>

using namespace std;
using namespace derby_stats;
using namespace state_stores;

using json = nlohmann::json;

typedef function<game_team& (game_state&)> team_state_getter;

optional<string> get_matching_key(const string& target_key);
string escape_regex(const string& value);

const team_state_getter home_team_selector = [](auto&& state) -> game_team& { return state.home_team; };
const team_state_getter away_team_selector = [](auto&& state) -> game_team& { return state.away_team; };

map<string, GameStateStore::state_mapper> mappers
{
	{ "ScoreBoard.CurrentGame.Team(1).Name", [](auto&& state, auto&& data) { state.home_team.name = data.value; }},
	{ "ScoreBoard.CurrentGame.Team(2).Name", [](auto&& state, auto&& data) { state.away_team.name = data.value; }},
	{ "ScoreBoard.CurrentGame.Team(1).Score", [](auto&& state, auto&& data) { state.home_team.score = stoi(data.value); }},
	{ "ScoreBoard.CurrentGame.Team(2).Score", [](auto&& state, auto&& data) { state.away_team.score = stoi(data.value); }},
};

GameStateStore::GameStateStore(const shared_ptr<ScoreboardConnector>& scoreboard_connector)
	: StateStore<game_state>(scoreboard_connector, ::mappers)
{
}
