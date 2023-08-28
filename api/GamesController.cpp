#include "GamesController.h"
#include "../state_stores/GameStateStore.h"

#include <vector>
#include <string>

using namespace derby_stats::api;
using namespace derby_stats::state_stores;

GamesController::GamesController(const shared_ptr<GameStateStore>& state_store)
{
	this->state_store = state_store;
}

vector<handler_definition> GamesController::get_handlers()
{
	return {};
	//{
	//	{ request_type::get, "/api/games", [this] { return this->get_game_state(); } },
	//};
}

string GamesController::get_game_state() const
{
	auto const state = this->state_store->get_state();

	return format("{} vs {}", state.home_team.score, state.away_team.score);
}
