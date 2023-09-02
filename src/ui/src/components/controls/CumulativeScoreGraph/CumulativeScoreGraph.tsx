import { Label, LineChart, Line, XAxis, YAxis, Tooltip } from 'recharts';

import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useEffect, useMemo, useState } from 'react';

type JamScore = {
    jamNumber: number,
    team1Score: number,
    team2Score: number,
};

type CumulativeScoreUpdateBody = {
    jamScores: JamScore[];
};

type CumulativeScoreUpdate = {
    dataType: "CumulativeScore",
    body: CumulativeScoreUpdateBody;
};

export const CumulativeScoreGraph = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [data, setData] = useState<JamScore[]>([]);

    const socket = useMemo(() => new WebSocket(`ws://${location.hostname}:8003/ws`), []);

    socket.addEventListener('open', () => {
        socket.send(JSON.stringify({
            messageType: "Subscribe",
            dataType: "CumulativeScore"
        }));
    });
    
    socket.addEventListener('message', (event) => {
        console.log(`Data received: ${JSON.stringify(event.data)}`);
        const update: CumulativeScoreUpdate = JSON.parse(event.data);
        console.log(update);
        //setData(update.body.jamScores);
    });

    useEffect(() => {
        setData([
            { jamNumber: 1, team1Score: 0, team2Score: 0 },
            { jamNumber: 2, team1Score: 4, team2Score: 1 },
            { jamNumber: 3, team1Score: 8, team2Score: 2 },
            { jamNumber: 4, team1Score: 12, team2Score: 4 },
            { jamNumber: 5, team1Score: 16, team2Score: 8 },
        ]);
    }, [setData])

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    return (
        <GraphContainer aspectRatio={.5}>
            <LineChart data={data}>
                <Line type="monotone" dot={false} stroke={team1Color} strokeWidth={3} dataKey="team1Score" name="White" />
                <Line type="monotone" dot={false} stroke='#00bb22' strokeWidth={3} dataKey="team2Score" name="Black" />
                <XAxis dataKey="jamNumber" name="Jam">
                    <Label value="Jam #" position="insideBottom" offset={-1} />
                </XAxis>
                <YAxis>
                    <Label value="Cumulative score" angle={-90} />
                </YAxis>
                <Tooltip />
            </LineChart>
        </GraphContainer>
    );
}