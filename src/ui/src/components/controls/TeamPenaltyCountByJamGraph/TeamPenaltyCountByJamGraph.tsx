import { Label, XAxis, YAxis, Tooltip, LineChart, Line, ReferenceLine } from 'recharts';

import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useState } from 'react';
import { useStateSocket } from '../../../hooks';

interface PenaltyCountByTeam {
    homeTeamCount: number,
    awayTeamCount: number,
}

type PenaltyCountByJam = Record<number, PenaltyCountByTeam>
type CountsByTeam = Record<number, PenaltyCountByJam>

type UpdateBody = {
    penaltyCountsByJamByTeam: CountsByTeam;
};

type PenaltiesUpdate = {
    dataType: "PenaltiesByType",
    body: UpdateBody;
};

export const TeamPenaltyCountByJamGraph = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [penaltyCountsByJamByTeam, setPenaltyCountsByJamByTeam] = useState<CountsByTeam>({ });

    useStateSocket<PenaltiesUpdate>("PenaltiesByType", update => {
        setPenaltyCountsByJamByTeam(update.body.penaltyCountsByJamByTeam);
    }, [setPenaltyCountsByJamByTeam]);

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    console.log(penaltyCountsByJamByTeam);

    const data = 
        Object.entries(penaltyCountsByJamByTeam)
            .flatMap(([period, jams]) =>
                Object.entries(jams).map(([jam, jamCounts]) => {
                    penaltyCountsByJamByTeam
                    return {
                        key: `${period}: ${jam}`,
                        period,
                        jam,
                        homeTeamCount: jamCounts.homeTeamCount,
                        awayTeamCount: jamCounts.awayTeamCount,
                    };
            }))
            .reduce((current, next) => {
                const cumulative = {
                    ...next,
                    homeTeamCount: current[current.length - 1].homeTeamCount + next.homeTeamCount,
                    awayTeamCount: current[current.length - 1].awayTeamCount + next.awayTeamCount,
                };

                return [...current, cumulative];
            }, [{ key: '0: 0', period: '0', jam: '0', homeTeamCount: 0, awayTeamCount: 0}])

    return (
        <GraphContainer aspectRatio={.5}>
            <LineChart data={data}>
                <Line type="monotone" dot={false} stroke={team1Color} strokeWidth={3} dataKey="homeTeamCount" name="White" />
                <Line type="monotone" dot={false} stroke='#00bb22' strokeWidth={3} dataKey="awayTeamCount" name="Black" />
                <XAxis dataKey="key" type="category" name="Jam">
                    <Label value="Jam #" position="insideBottom" offset={-1} />
                </XAxis>
                <YAxis>
                    <Label value="Cumulative penalty count" angle={-90} />
                </YAxis>
                <Tooltip />
                <ReferenceLine x="2: 1" />
            </LineChart>
        </GraphContainer>
    );
}