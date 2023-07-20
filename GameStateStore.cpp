#include "GameStateStore.h"

#include <functional>
#include <ranges>
#include <string>
#include <utility>

using namespace std;
using namespace derby_stats;

typedef function<void(game_state&, string&&)> state_mapper;

#define MAPPER(k, s) { k, [](game_state& state, string&& value) s }

map<string, state_mapper> mappers = {
	MAPPER("ScoreBoard.CurrentGame.Team(1).Score", { state.home_team.score = stoi(value); }),
	MAPPER("ScoreBoard.CurrentGame.Team(2).Score", { state.away_team.score = stoi(value); }),
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
	if (!mappers.contains(key))
		return;

	auto const mapper = mappers[key];
	mapper(this->state, move(value));
}

game_state GameStateStore::get_state()
{
	return this->state;
}