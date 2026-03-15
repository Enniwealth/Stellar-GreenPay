/**
 * pages/leaderboard.tsx — Top donors ranked by total XLM given
 */
import LeaderboardTable from "@/components/LeaderboardTable";
import Link from "next/link";

export default function LeaderboardPage() {
  return (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 py-10 animate-fade-in">

      <div className="text-center mb-10">
        <div className="text-5xl mb-4">🏆</div>
        <h1 className="font-display text-3xl sm:text-4xl font-bold text-forest-900 mb-3">
          Top Climate Donors
        </h1>
        <p className="text-[#5a7a5a] max-w-xl mx-auto font-body leading-relaxed">
          Celebrating the community members who are driving the most impact. Every XLM donated is recorded permanently on the Stellar blockchain.
        </p>
      </div>

      {/* Badge legend */}
      <div className="card mb-8 bg-forest-50 border-forest-200">
        <p className="font-display font-semibold text-forest-900 mb-3 text-center">Impact Badge Tiers</p>
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 text-center">
          {[
            { emoji: "🌱", name: "Seedling",       req: "10+ XLM" },
            { emoji: "🌳", name: "Tree",           req: "100+ XLM" },
            { emoji: "🌲", name: "Forest",         req: "500+ XLM" },
            { emoji: "🌍", name: "Earth Guardian", req: "2,000+ XLM" },
          ].map(b => (
            <div key={b.name} className="bg-white rounded-xl p-3 border border-forest-100">
              <p className="text-2xl mb-1">{b.emoji}</p>
              <p className="text-xs font-semibold text-forest-900 font-body">{b.name}</p>
              <p className="text-xs text-[#8aaa8a] font-body">{b.req}</p>
            </div>
          ))}
        </div>
      </div>

      {/* Table */}
      <LeaderboardTable limit={50} />

      <div className="mt-10 text-center">
        <p className="text-[#5a7a5a] text-sm mb-4 font-body">Want to see your name here?</p>
        <Link href="/projects" className="btn-primary">🌱 Start Donating</Link>
      </div>
    </div>
  );
}
