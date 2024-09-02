export default function downloadFile(fileName: string, data: string) {
    const link = document.createElement("a");
    link.download = fileName;
    link.href = `data:text/plain;charset=utf-8,${encodeURIComponent(data)}`;
    link.style.display = "none";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}
