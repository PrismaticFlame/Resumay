

export interface Application {
    id: number;
    company: string;
    position: string;
    website: string | null;
    job_description: string | null;
    username: string | null;
    status: string;
    resume: number | null;
    notes: string | null;
    refer: string | null;
    apply_date: string;
    resume_label?: string | null;
  }

export interface Resume {
    id: number;
    label: string;
    created_at: string;
    uses: number;
  }

export interface ApplicationSummary {
  id: number;
  company: string;
  position: string;
  status: string;
  apply_date: string;
}