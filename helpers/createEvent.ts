import { PROGRAM_ID } from "@/utils.ts/solana";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { Transaction, SystemProgram, TransactionInstruction, Keypair } from "@solana/web3.js";

export const createEvent = async (eventId: number, description: string) => {
    const { connection } = useConnection();
    const { publicKey, sendTransaction } = useWallet();

    if (!publicKey) {
        throw new Error("Wallet not connected!");
    }

    const eventAccount = Keypair.generate();

    const transaction = new Transaction().add(
        new TransactionInstruction({
            keys: [
                { pubkey: eventAccount.publicKey, isSigner: true, isWritable: true },
                { pubkey: publicKey, isSigner: true, isWritable: false },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId: PROGRAM_ID,
            data: Buffer.from(JSON.stringify({ eventId, description })), // serializing data
        })
    );

    try {
        const signature = await sendTransaction(transaction, connection, {
            signers: [eventAccount],
        });
        console.log("Transaction successful! Signature:", signature);
    } catch (error) {
        console.error("Transaction failed:", error);
    }
};
