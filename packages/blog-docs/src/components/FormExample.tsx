import "@site/src/css/index.css";

export default function FormExample({
  url,
  height,
  state,
}: {
  url: string;
  height: number;
  state: "neutral" | "good" | "bad";
}) {
  return (
    <>
      <div
        className={`p-2 rounded-xl overflow-hidden border-2 border-solid bg-slate-50 dark:bg-slate-900 ${state === "neutral" ? "border-primary-300 dark:border-primary-900" : state === "good" ? "border-green-400 dark:border-green-900" : "border-red-400 dark:border-red-900"}`}
      >
        <iframe src={url} height={height} width="100%" />
      </div>

      <p
        className={`mt-2 text-sm uppercase tracking-widest ${state === "neutral" ? "text-primary-500" : state === "good" ? "text-green-500" : "text-red-500"}`}
      >
        {state === "neutral"
          ? "Example form"
          : state === "good"
            ? "Good example"
            : "Bad example"}
      </p>
    </>
  );
}
