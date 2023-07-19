#include "DerbyStats.h"
#include "FileServe.h"
#include "GamesController.h"

#include <App.h>
#include <filesystem>
#include <cxxopts.hpp>

using namespace std;
using namespace derby_stats;

std::tuple<int, std::string> parse_options(int argument_count, char** arguments);

int main(int argument_count, char** arguments)
{
	auto const serve_path = filesystem::absolute(filesystem::current_path().concat("/ui"));

	auto const [ port, scoreboard_url ] = parse_options(argument_count, arguments);

	auto const scoreboard_connector = 
		ScoreboardConnector::create()
			->connect(scoreboard_url);

	uWS::App()
		.get("/*", file_serve::handle<false>(serve_path))
		.get("/api/*", [](auto* response, auto*)
		{
			response->end("Hello");
		})
		.listen(port, [port](const auto* listen_socket)
		{
			if (listen_socket)
			{
				cout << "Listening on port " << port << endl;
			}
		})
		.run();

	return 0;
}

std::tuple<int, std::string> parse_options(int argument_count, char** arguments)
{
	auto options = cxxopts::Options("DerbyStats");

	options.add_options()
		("u,url", "Scoreboard URL", cxxopts::value<std::string>()->default_value("http://localhost:8000/"))
		("p,port", "DerbyStats port", cxxopts::value<int>()->default_value("8001"))
		;

	auto parsed_options = options.parse(argument_count, arguments);

	return std::make_tuple(
		parsed_options["port"].as<int>(),
		parsed_options["url"].as<std::string>());
}
