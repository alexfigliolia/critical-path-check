import { r as e } from "./threeFiber-C7PdTxs4.js";
import { g as o } from "./threeDrei-jPcrdQkF.js";
import { L as r } from "./index-bWaCC1tV.js";
const s = () => {
  const s = e.useRef(!1),
    { progress: t, total: a, loaded: m } = o(),
    f = e.useMemo(() => 100 === t && a === m, [t, a, m]);
  e.useEffect(() => {
    f && !s.current && ((s.current = !0), r.onLoad());
  }, [f]);
};
export { s as u };
//# sourceMappingURL=useLabsLoader-qtw6trdU.js.map
