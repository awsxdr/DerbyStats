import { Label, LineChart, Line, XAxis, YAxis, Tooltip, ReferenceLine } from 'recharts';

import { useDarkThemeContext } from '@contexts';
import { GraphContainer } from '@atoms';
import { useState } from 'react';
import { useStateSocket } from '@hooks';

type JamScore = {
    periodNumber: number,
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

    useStateSocket<CumulativeScoreUpdate>("CumulativeScore", update => {
        console.log(update);
        setData(update.body.jamScores.sort((a, b) => a.periodNumber < b.periodNumber || (a.periodNumber === b.periodNumber && a.jamNumber < b.jamNumber) ? -1 : 1));
    }, [setData]);

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    return (
        <GraphContainer aspectRatio={.5}>
            <LineChart data={data.map(i => ({ ...i, key: `${i.periodNumber}: ${i.jamNumber}` }))}>
                <Line type="monotone" dot={false} stroke={team1Color} strokeWidth={3} dataKey="team1Score" name="White" />
                <Line type="monotone" dot={false} stroke='#00bb22' strokeWidth={3} dataKey="team2Score" name="Black" />
                <XAxis dataKey="key" name="Jam">
                    <Label value="Jam #" position="insideBottom" offset={-1} />
                </XAxis>
                <YAxis>
                    <Label value="Cumulative score" angle={-90} />
                </YAxis>
                <Tooltip />
                <ReferenceLine x="2: 1" />
            </LineChart>
        </GraphContainer>
    );
}