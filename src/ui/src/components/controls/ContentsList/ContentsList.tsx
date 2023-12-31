import { Tab, TabId, Tabs } from "@blueprintjs/core";
import { ReactNode } from "react";
import { CumulativeScoreGraph, JamCountByJammerGraph, NetScoreByJammerGraph, TeamPenaltyCountByJamGraph, TeamPenaltyCountByTypeGraph, TeamPenaltyShareByTypeChart } from "..";

export enum GraphTabIds {
    CumulativeScore,
    TeamPenaltyShareByType,
    TeamPenaltyCountByType,
    TeamPenaltyCountByJam,
    JamCountByJammer,
    NetScoreByJammer,
}

enum Sections {
    Scores = "Scores",
    Penalties = "Penalties",
    Jammers = "Jammers",
}

type TabMapEntry = {
    title: string,
    section: Sections,
    component: () => ReactNode
}

export const GraphTabs = new Map<GraphTabIds, TabMapEntry>([
    [GraphTabIds.CumulativeScore, { title: "Cumulative score", section: Sections.Scores, component: () => (<CumulativeScoreGraph />) }],
    [GraphTabIds.TeamPenaltyShareByType, { title: "Team penalty share by type", section: Sections.Penalties, component: () => (<TeamPenaltyShareByTypeChart />)}],
    [GraphTabIds.TeamPenaltyCountByType, { title: "Team penalty count by type", section: Sections.Penalties, component: () => (<TeamPenaltyCountByTypeGraph />)}],
    [GraphTabIds.TeamPenaltyCountByJam, { title: "Team cumulative penalty count by jam", section: Sections.Penalties, component: () => (<TeamPenaltyCountByJamGraph />)}],
    [GraphTabIds.JamCountByJammer, { title: "Jam count by jammer", section: Sections.Jammers, component: () => (<JamCountByJammerGraph />)}],
    [GraphTabIds.NetScoreByJammer, { title: "Net score by jammer", section: Sections.Jammers, component: () => (<NetScoreByJammerGraph />)}],
]);

interface IContentsListProps {
    onTabSelected: (tabId: TabId) => void;
    className?: string;
    selectedTabId: TabId;
}

export const ContentsList = ({onTabSelected, className, selectedTabId}: IContentsListProps) => {
    return (
        <Tabs className={className} renderActiveTabPanelOnly animate vertical onChange={onTabSelected} selectedTabId={selectedTabId} fill>
            {
                Object.keys(Sections)
                .map(key => {
                    const sectionHeader: string = Sections[key as keyof typeof Sections];

                    return [ <h5>{sectionHeader}</h5> ].concat(
                        Array.from(GraphTabs.entries())
                            .filter(([_, v]) => v.section === key)
                            .map(([k, v]) => (<Tab id={k} key={k} title={v.title} />)));
                })
            }
        </Tabs>
    );
};