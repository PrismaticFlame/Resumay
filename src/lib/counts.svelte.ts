import { invoke } from "@tauri-apps/api/core";

export const counts = $state({ applications: 0, resumes: 0});
export async function refreshCounts() {
    try {
            const [a, r] = await Promise.all([
                invoke<number>("count_applications"),
                invoke<number>("count_resumes"),
            ]);
            counts.applications = a;
            counts.resumes = r;
        } catch (err) {
            console.log(err);
        }
}
