#include "ScoreboardConnector.h"
#include "Exceptions.h"

#include <WebSocket.h>
//#include <websocketpp/>

using namespace derby_stats;

unique_ptr<ScoreboardConnector> ScoreboardConnector::create()
{
	return std::make_unique<DisconnectedScoreboardConnector>();
}

unique_ptr<ScoreboardConnector> DisconnectedScoreboardConnector::connect(string url)
{
	//uWS::WebSocket::
	
	//return std::make_unique<ConnectedScoreboardConnector>(nullptr);
	return nullptr;
}

unique_ptr<ScoreboardConnector> ConnectedScoreboardConnector::connect(string url)
{
	throw AlreadyConnectedException();
}

