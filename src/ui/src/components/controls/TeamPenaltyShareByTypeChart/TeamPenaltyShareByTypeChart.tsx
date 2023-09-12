import { useDarkThemeContext } from '../../../contexts';
import { GraphContainer } from '../../atoms/GraphContainer/GraphContainer';
import { useState } from 'react';
import { useStateSocket } from '../../../hooks';
import { PenaltyLine } from '../../atoms';
import { IIndexer, PenaltyCounts } from '../../../commonTypes';

class CountsByTeam implements IIndexer<number, PenaltyCounts> {
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

export const TeamPenaltyShareByTypeChart = () => {

    const { useDarkTheme } = useDarkThemeContext();
    const [penaltyCountsByTypeByTeam, setPenaltyCountsByTypeByTeam] = useState<CountsByTeam>({1: new PenaltyCounts(), 2: new PenaltyCounts()});

    useStateSocket<PenaltiesUpdate>("PenaltiesByType", update => {
        setPenaltyCountsByTypeByTeam(update.body.penaltyCountsByTypeByTeam);
    }, [setPenaltyCountsByTypeByTeam]);

    const team1Color = useDarkTheme ? '#ffddaa' : '#ff4400';

    const getTeamTotal = (teamId: number) => 
        Object.values(penaltyCountsByTypeByTeam[teamId]).reduce((c, v) => c + v, 0);

    return (
        <GraphContainer aspectRatio={.5}>
            <div>
                <PenaltyLine team1Color={team1Color} team2Color="#0b2" team1Count={getTeamTotal(1)} team2Count={getTeamTotal(2)} title="Total" />
                {
                    Object.keys(penaltyCountsByTypeByTeam[1])
                        .map(k => ({
                            code: k,
                            team1Count: penaltyCountsByTypeByTeam[1][k],
                            team2Count: penaltyCountsByTypeByTeam[2][k]
                        }))
                        .map(line => (
                            <PenaltyLine team1Color={team1Color} team2Color="#00bb22" team1Count={line.team1Count} team2Count={line.team2Count} title={line.code} />
                        ))
                }
            </div>
        </GraphContainer>
    );
}