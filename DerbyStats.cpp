#include "DerbyStats.h"
#include <App.h>
#include <fstream>
#include <sstream>
#include <filesystem>

using namespace std;

constexpr int port = 8001;

bool is_parent_path(filesystem::path parent_path, filesystem::path path);

int main()
{
	auto const serve_path = filesystem::absolute(filesystem::current_path().concat("/ui"));

	uWS::App()
		.get("/*", [serve_path](auto *response, auto *request)
		{
				auto url = request->getUrl();
				url.remove_prefix(1);
				string url_string("ui/");
				url_string = url_string.append(url);
				auto request_path = filesystem::absolute(url_string);
			
			if(request_path == "")
			{
				request_path = serve_path;
			}
				if(!is_parent_path(serve_path, request_path))
				{
					response->writeStatus("404");
					response->endWithoutBody();
					return;
				}
				
			if(!request_path.has_filename() || !filesystem::exists(request_path))
			{
				request_path = "ui/index.html";
			}
			const ifstream file_stream(request_path);
			stringstream buffer;
			
			buffer << file_stream.rdbuf();
			response->end(buffer.str());
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

bool is_parent_path(const filesystem::path parent_path, const filesystem::path path)
{
	auto const empty_path = filesystem::path();
	auto current_path = path;

	while(current_path != empty_path)
	{
		if(current_path == parent_path)
			return true;

		auto const last_path = current_path;
		current_path = current_path.parent_path();
		if (last_path == current_path)
			break;
	}

	return false;
}