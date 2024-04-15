

export type CardDetail = {
	media: string;
	name: string;
	description: string;
	reference: number;
};

// Definir un tipo para UserCard como una tupla [string, CardDetail]
export type CardProps = [string, CardDetail];

