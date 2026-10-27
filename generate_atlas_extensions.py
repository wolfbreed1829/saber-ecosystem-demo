"""Original satellite district illustrations for the conceptual atlas."""
from pathlib import Path
root=Path(__file__).parent
# The extra entries are intentionally illustrated as satellite districts, not a surveyed site.
p = root / 'public/atlas.svg'
svg=p.read_text(encoding='utf-8')
campus='''<g transform="translate(50 18)"><path d="M0 75 110 20 220 75 110 130Z" fill="#071d2d" stroke="#246489"/><path d="M32 65 105 29 163 58 90 94Z" fill="#1f6687" stroke="#72ceef"/><path d="M32 65v27l58 29V94Z" fill="#103c55" stroke="#3c86a7"/><path d="M90 94v27l73-36V58Z" fill="#0e2d45" stroke="#3c86a7"/><path d="M70 44v30l22 11 1-30Z" fill="#164b69"/><path d="M116 70 164 46 192 60 144 84Z" fill="#236584" stroke="#68c7e9"/><path d="M144 84v26l48-24V60Z" fill="#103f59" stroke="#3d91b7"/><path d="m50 104 23 11M56 88l23 11m26 4 19-9m-19-3 19-9" stroke="#83dcff" stroke-width="2"/></g>'''
civic='''<g transform="translate(900 515)"><path d="M-70 68 20 23 120 73 30 118Z" fill="#071d2d" stroke="#246489"/><path d="M-42 48 21 17 84 48 21 80Z" fill="#255f7d" stroke="#83ddff"/><path d="M-42 48v42l63 31V80Z" fill="#133f57" stroke="#3884a6"/><path d="M21 80v41l63-31V48Z" fill="#0d2c41" stroke="#3884a6"/><path d="M-31 61v30m15-23v30m15-23v30m15-23v30M34 85v24m16-32v24m16-32v24" stroke="#80d6f5" stroke-width="3"/><path d="M21 17V-4m0 2 27 10-27 6" stroke="#55caff" fill="#164d6a"/></g>'''
(root/'public/atlas-extended.svg').write_text(svg.replace('</svg>',campus+civic+'</svg>'),encoding='utf-8')
