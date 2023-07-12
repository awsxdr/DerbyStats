#include "DerbyStats.h"
#include "FileServe.h"
#include "GamesController.h"

#include <App.h>
#include <fstream>
#include <filesystem>

using namespace std;
using namespace derby_stats;

constexpr int port = 8001;


int main()
{
	auto const serve_path = filesystem::absolute(filesystem::current_path().concat("/ui"));

	uWS::App()
		.get("/*", file_serve::handle<false>(serve_path))
		.get("/api/*", [](auto* response, auto*)
		{
			response->end("Hello");
		})
		.listen(port, [](const auto* listen_socket)
		{
			if (listen_socket)
			{
				cout << "Listening on port " << port << endl;
			}
		})
		.run();

	return 0;
}

