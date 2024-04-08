import { ReactNode } from 'react';

function LegalSection({title, children}: {title: string, children: ReactNode}){
    return (
        <div>
            <h2 className=" text-2xl sm:text-3xl font-semibold mt-8">{title}</h2>
            <p className="mt-2 text-xs sm:text-base">
            {children}
            </p>
        </div>
    );
    
}

export { LegalSection };