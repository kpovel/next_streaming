import { Suspense } from "react";
export const dynamic = "force-dynamic";

export default async function Home() {
  console.time("first");
  const slowResponse = await fetch("http://localhost:42069/slow");
  const text = await slowResponse.text();

  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      {text}
      <Suspense fallback={<div>loading a brand new response</div>}>
        <SlowComponent />
      </Suspense>
    </main>
  );
}

async function SlowComponent() {
  console.timeEnd("first");
  console.time("second");
  const slowResponse = await fetch("http://localhost:42069/a-brand-new");
  const text = await slowResponse.text();
  console.timeEnd("second");

  return <div>Second slow request: {text}</div>;
}
