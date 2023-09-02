import styles from './GraphContainer.module.scss';

import React, { ReactElement, useCallback, useEffect, useRef, useState } from 'react';

interface IGraphContainerProps {
    children: ReactElement;
    aspectRatio: number;
};

export const GraphContainer = ({ children, aspectRatio }: IGraphContainerProps) => {
    const [graphWidth, setGraphWidth] = useState(800);

    const graphWrapperRef =  useRef<HTMLDivElement>(null);

    const scaleGraphToWindow = useCallback(() => {
        const width = graphWrapperRef.current?.clientWidth ?? 800;
        console.log(width);
        setGraphWidth(width);
    }, [setGraphWidth]);

    const handleResize = useCallback((_event: UIEvent) => {
        scaleGraphToWindow();
    }, [scaleGraphToWindow]);

    useEffect(scaleGraphToWindow, []);

    window.addEventListener('resize', handleResize);

    const childProps = { ...children.props, width: graphWidth, height: graphWidth * aspectRatio };
    return (
        <div className={styles.graph} ref={graphWrapperRef}>
            {
                React.isValidElement(children) && React.cloneElement(children, childProps)
            }
        </div>
    );
}