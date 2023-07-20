#include "GamesController.h"
#include "../GameStateStore.h"

#include <vector>
#include <string>

using namespace derby_stats::api;

GamesController::GamesController(unique_ptr<GameStateStore>& state_store)
{
	this->state_store = move(state_store);
}

vector<handler_definition> GamesController::get_handlers()
{
	return
	{
		{ http_verb::get, "/api/games", [this] { return this->get_game_state(); } },
	};
}

string GamesController::get_game_state()
{
	auto const state = this->state_store->get_state();

	return format("{} vs {}", state.home_team.score, state.away_team.score);
}
