import "@site/src/css/index.css";

export default function FormExample({
  url,
  height,
}: {
  url: string;
  height: number;
}) {
  return (
    <>
      <div className="rounded-xl border-2 border-solid border-primary-900 overflow-hidden p-2 bg-slate-900">
        <iframe src={url} height={height} width="100%" />
      </div>

      <p className="mt-2 text-sm text-primary-500 uppercase tracking-widest">
        Form example
      </p>
    </>
  );
}
