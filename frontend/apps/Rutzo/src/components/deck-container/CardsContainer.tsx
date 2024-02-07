import React, { ReactNode } from 'react';

interface CardsContainerProps {
    className?: string,
    children?: ReactNode,
    title?: string,
}

const CardsContainer: React.FC<CardsContainerProps> = ({className, children, title}) => {
    const containerClassName = `flex flex-col items-center border-2 rounded-3xl p-4 w-4/5 justify-center my-4 ${className || ''}`;
    return (
        <div className={containerClassName}>

            {title && <h3 className="text-xl my-4 font-medium" >{title}</h3>}

            {children}
        </div>
    )
}

export default CardsContainer;
