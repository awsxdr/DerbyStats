import { Application } from "express";
import ApiController from "./api-controller.js";
import SocketServer, { IUpdateProvider } from "./socket-server.js";
import { ScoreboardConnector, State } from "../scoreboard-connector.js";

type JamScore = {
    jamNumber: number,
    team1Score: number,
    team2Score: number,
}

type CumulativeScore = {
    jamScores: JamScore[],
}

const TotalScoreRegex = /ScoreBoard\.CurrentGame\.Period\((?<period>\d+)\)\.Jam\((?<jam>\d+)\)\.TeamJam\((?<team>\d+)\)\.TotalScore/;

export class CumulativeScoreController extends ApiController implements IUpdateProvider {
    #state: CumulativeScore = { jamScores: [] };
    #socketServer: SocketServer;

    constructor(app: Application, scoreboard: ScoreboardConnector, socketServer: SocketServer) {
        super();

        socketServer.setUpdateProvider("CumulativeScore", this as CumulativeScoreController);

        scoreboard.listenForTopics([ "ScoreBoard.CurrentGame.Period(*).Jam(*).TeamJam(*).TotalScore" ]);
        scoreboard.addUpdateHandler(
            t => !!t.match(TotalScoreRegex),
             this.#handleStateUpdate.bind(this));

        app.get('/api/cumulative-score', (_request, response) => {
            response.status(200).send(this.#state);
        });

        this.#socketServer = socketServer;

        console.log("CumulativeScoreController created");
    }

    getState(): CumulativeScore {
        return this.#state;
    }

    #handleStateUpdate(state: State) {
        this.#state = {
            jamScores:
                Array.from(state.entries())
                .map(([k, v]) => ({ 
                    matches: k.match(TotalScoreRegex),
                    value: v
                }))
                .filter(({ matches, value: _value }) => !!matches)
                .map(({ matches, value }) => ({
                    period: parseInt(matches?.groups?.period ?? "0"),
                    jam: parseInt(matches?.groups?.jam ?? "0"),
                    team: parseInt(matches?.groups?.team ?? "0"),
                    score: value as number
                }))
                .reduce((current, item) => {
                    if(!current[item.jam]) {
                        current[item.jam] = {
                            jamNumber: item.jam,
                            team1Score: 0,
                            team2Score: 0,
                        };
                    }

                    if(item.team == 1) {
                        current[item.jam].team1Score = item.score;
                    } else {
                        current[item.jam].team2Score = item.score;
                    }

                    return current;
                }, [] as JamScore[])
        };

        this.#socketServer.sendUpdate("CumulativeScore", this.#state);
    }
}