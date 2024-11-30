import { Button } from "../ui/button";

export const AppHeader = () => {
  return (
    <header className="w-full flex flex-row justify-between items-center px-20 py-10">
      <section>
        <h1 className="text-3xl tracking-[-1px] font-bold">futsol</h1>
      </section>

      <section className="flex gap-4 items-center">
        <Button className="text-md font-semibold">Play</Button>

        <Button className="text-md font-semibold">About</Button>

        <Button className="text-md font-semibold">Connect</Button>
      </section>
    </header>
  );
};
