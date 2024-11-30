import { Connection, clusterApiUrl, PublicKey } from "@solana/web3.js";

const CLUSTER = "devnet"; 
export const connection = new Connection(clusterApiUrl(CLUSTER), "confirmed");

export const PROGRAM_ID = new PublicKey("6o4S2WgWWQrbmLvU7sCWcqqtUh9tD7YKNC69xNYgobHu");
