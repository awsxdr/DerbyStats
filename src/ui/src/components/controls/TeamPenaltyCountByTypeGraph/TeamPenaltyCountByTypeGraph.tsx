import { Label, XAxis, YAxis, Tooltip, BarChart, Bar } from 'recharts';

import { useDarkThemeContext } from '../../../contexts/';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useState } from 'react';
import { useStateSocket } from '../../../hooks';
import { PenaltyCounts } from '../../../commonTypes';

class CountsByTeam {
    1: PenaltyCounts;
    2: PenaltyCounts;

    [key: number]: PenaltyCounts;
};

type UpdateBody = {
    penaltyCountsByTypeByTeam: CountsByTeam;
};

type PenaltiesUpdate = {
    dataType: "PenaltiesByType",
    body: UpdateBody;
};

export const TeamPenaltyCountByTypeGraph = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [penaltyCountsByTypeByTeam, setPenaltyCountsByTypeByTeam] = useState<CountsByTeam>({1: new PenaltyCounts(), 2: new PenaltyCounts()});

    useStateSocket<PenaltiesUpdate>("PenaltiesByType", update => {
        setPenaltyCountsByTypeByTeam(update.body.penaltyCountsByTypeByTeam);
    }, [setPenaltyCountsByTypeByTeam]);


    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    const data = 
        Object.keys(penaltyCountsByTypeByTeam[1])
        .map(k => ({
            code: k,
            team1Count: penaltyCountsByTypeByTeam[1][k],
            team2Count: penaltyCountsByTypeByTeam[2][k]
        }));

    return (
        <GraphContainer aspectRatio={.5}>
            <BarChart data={data}>
                <XAxis dataKey="code" name="Penalty code">
                    <Label value="Penalty code" position="insideBottom" offset={-1} />
                </XAxis>
                <YAxis>
                    <Label value="Count" angle={-90} />
                </YAxis>
                <Tooltip />
                <Bar dataKey="team1Count" fill={team1Color} name="White" />
                <Bar dataKey="team2Count" fill="#0b2" name="Black" />
            </BarChart>
        </GraphContainer>
    );
}