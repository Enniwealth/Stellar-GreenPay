/**
 * components/ProjectCard.tsx
 */
import Link from "next/link";
import type { ClimateProject } from "@/utils/types";
import { formatXLM, formatCO2, progressPercent, statusClass, statusLabel, CATEGORY_ICONS } from "@/utils/format";

export default function ProjectCard({ project }: { project: ClimateProject }) {
  const pct = progressPercent(project.raisedXLM, project.goalXLM);

  return (
    <Link href={`/projects/${project.id}`}>
      <div className="card-hover group animate-fade-in flex flex-col h-full">

        {/* Category icon + badges */}
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2">
            <div className="w-10 h-10 rounded-xl bg-forest-100 flex items-center justify-center text-xl border border-forest-200">
              {CATEGORY_ICONS[project.category] || "🌿"}
            </div>
            <div>
              <p className="text-xs text-[#5a7a5a] font-body">{project.category}</p>
              <p className="text-xs text-[#8aaa8a] font-body">{project.location}</p>
            </div>
          </div>
          <div className="flex items-center gap-1.5">
            {project.verified && <span className="badge-verified text-xs px-2 py-0.5 rounded-full bg-green-50 text-green-700 border border-green-200 font-body">✓ Verified</span>}
            <span className={statusClass(project.status)}>{statusLabel(project.status)}</span>
          </div>
        </div>

        {/* Name & description */}
        <h3 className="font-display font-semibold text-forest-900 text-base leading-snug mb-2 group-hover:text-forest-600 transition-colors line-clamp-2">
          {project.name}
        </h3>
        <p className="text-[#5a7a5a] text-sm leading-relaxed line-clamp-3 mb-4 flex-1 font-body">
          {project.description}
        </p>

        {/* Progress bar */}
        <div className="mb-4">
          <div className="flex justify-between text-xs text-[#8aaa8a] mb-1.5 font-body">
            <span>{formatXLM(project.raisedXLM)} raised</span>
            <span>{pct}% of {formatXLM(project.goalXLM)}</span>
          </div>
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${pct}%` }} />
          </div>
        </div>

        {/* Stats row */}
        <div className="flex items-center justify-between pt-3 border-t border-[rgba(34,114,57,0.07)]">
          <div className="flex items-center gap-3 text-xs text-[#5a7a5a] font-body">
            <span>👥 {project.donorCount} donors</span>
            <span>♻️ {formatCO2(project.co2OffsetKg)}</span>
          </div>
          <span className="text-xs font-semibold text-forest-600 font-body group-hover:text-forest-700">
            Donate →
          </span>
        </div>
      </div>
    </Link>
  );
}
