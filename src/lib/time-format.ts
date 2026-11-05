// helper function to format timestamps in locale datestring formats so that it's easier to read.
export function timeFormat(ts: string) {
    if (!ts) return "N/A";

    // const d = new Date(ts.replace(" ", "T") + "Z");
    // return d.toLocaleString("en-US", { dateStyle: "medium", timeStyle: "short" });

    // got cute with this, doing it in one line, but is confusing to read to so i'll leave the above version as well
    return new Date(ts.replace(" ", "T") + "Z").toLocaleString("en-US", { dateStyle: "medium", timeStyle: "short" });
}