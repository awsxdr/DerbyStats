#include "DerbyStats.h"
#include "FileServe.h"
#include "ScoreboardConnector.h"
#include "api/GamesController.h"
#include "api/RostersController.h"
#include "Logger.hpp"

#include <App.h>
#include <filesystem>
#include <cxxopts.hpp>

#include "state_stores/GameStateStore.h"
#include "state_stores/RostersStateStore.h"

using namespace std;
using namespace derby_stats;
using namespace state_stores;

const string default_log_level = "error";
const string default_port = "8001";
const string default_scoreboard_endpoint = "localhost:8000";

std::tuple<int, std::string, LOG_LEVEL> parse_options(int argument_count, char** arguments);

int main(const int argument_count, char** arguments)
{
	auto const serve_path = filesystem::absolute(filesystem::current_path().concat("/ui"));

	auto const [ port, scoreboard_url, log_level ] = parse_options(argument_count, arguments);

	Logger::set_log_level(log_level);

	auto const scoreboard_connector = 
		ScoreboardConnector::create()
			->connect(scoreboard_url);

	auto app = uWS::App()
		.get("/*", file_serve::handle<false>(serve_path))
		.listen(port, [port](const auto* listen_socket)
			{
				if (listen_socket)
				{
					cout << "Listening on port " << port << endl;
				}
			});

	auto const game_state_store = make_shared<GameStateStore>(scoreboard_connector);
	auto games_controller = api::GamesController(game_state_store);
	games_controller.init_endpoints(app);

	auto const rosters_state_store = make_shared<RostersStateStore>(scoreboard_connector);
	auto rosters_controller = api::RostersController(rosters_state_store);
	rosters_controller.init_endpoints(app);

	app.run();

	return 0;
}

std::tuple<int, std::string, LOG_LEVEL> parse_options(int argument_count, char** arguments)
{
	auto options = cxxopts::Options("DerbyStats");

	options.add_options()
		("u,url", "Scoreboard URL", cxxopts::value<std::string>()->default_value(default_scoreboard_endpoint))
		("p,port", "DerbyStats port", cxxopts::value<int>()->default_value(default_port))
		("l,loglevel", "Logging level", cxxopts::value<string>()->default_value(default_log_level))
		;

	auto parsed_options = options.parse(argument_count, arguments);

	auto const log_level_string = parsed_options["loglevel"].as<string>();

	function<LOG_LEVEL(string)> parse_log_level = [&](const string& value) -> LOG_LEVEL
	{
		return
			value == "fatal" ? LOG_LEVEL_FATAL
			: value == "error" ? LOG_LEVEL_ERROR
			: value == "warn" ? LOG_LEVEL_WARN
			: value == "info" ? LOG_LEVEL_INFO
			: value == "debug" ? LOG_LEVEL_DEBUG
			: value == "trace" ? LOG_LEVEL_TRACE
			: parse_log_level(default_log_level);
	};

	auto const log_level = parse_log_level(log_level_string);

	return std::make_tuple(
		parsed_options["port"].as<int>(),
		parsed_options["url"].as<std::string>(),
		log_level);
}
