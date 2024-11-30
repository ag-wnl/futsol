import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import "@solana/wallet-adapter-react-ui/styles.css";
import { Button } from "../ui/button";

export const AppHeader = () => {
  return (
    <header className="w-full flex flex-row justify-between items-center px-20 py-10">
      <section className="flex gap-2">
        <h1 className="text-3xl tracking-[-1px] font-bold">futsol</h1>
        <span className="text-sm">[devnet]</span>
      </section>

      <section className="flex gap-4 items-center">
        <Button className="font-semibold rounded-sm">Play</Button>

        <Button className="font-semibold rounded-sm">About</Button>

        <WalletMultiButton style={{backgroundColor:'#9e6bdb'}} />
      </section>
    </header>
  );
};
