"""Original conceptual recovery-campus illustration, not an engineering site plan."""
from pathlib import Path
from html import escape

root = Path(__file__).parent
parts = ['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 520" role="img" aria-label="Conceptual recycling, waste management, and compost and mulch campus">', '<defs><pattern id="recovery-grid" width="40" height="40" patternUnits="userSpaceOnUse"><path d="M40 0H0V40" fill="none" stroke="#16415b" stroke-width=".6"/></pattern></defs><rect width="1000" height="520" fill="url(#recovery-grid)" opacity=".35"/>']

def iso(x,y,z=0): return (500+(x-y)*.8, 55+(x+y)*.4-z)
def points(coords): return ' '.join(f'{x:.1f},{y:.1f}' for x,y in coords)
def poly(coords, fill, stroke='#3f87ad'):
    parts.append(f'<polygon points="{points(coords)}" fill="{fill}" stroke="{stroke}" stroke-width="1.3"/>')
def box(x,y,w,d,h,roof='#1d648a'):
    poly([iso(x,y),iso(x+w,y),iso(x+w,y,h),iso(x,y,h)],'#103650')
    poly([iso(x+w,y),iso(x+w,y+d),iso(x+w,y+d,h),iso(x+w,y,h)],'#0b263c')
    poly([iso(x,y,h),iso(x+w,y,h),iso(x+w,y+d,h),iso(x,y+d,h)],roof,'#6ec7f0')
def label(x,y,text,color='#acdef5',size=12):
    parts.append(f'<text x="{x}" y="{y}" fill="{color}" font-family="monospace" font-size="{size}" letter-spacing="1">{escape(text)}</text>')

poly([iso(0,0),iso(520,0),iso(520,520),iso(0,520)],'#081c2b')
for v in [240,270]:
    parts.append(f'<polyline points="{points([iso(v,0),iso(v,520)])}" stroke="#2b688a" stroke-width="2" fill="none"/>')
parts.append(f'<polyline points="{points([iso(0,265),iso(520,265)])}" stroke="#3c829f" stroke-width="4" fill="none" stroke-dasharray="8 9"/>')
parts.append('<g data-facility="0">')
box(35,40,180,135,65)
for x in [55,95,135,175]:
    box(x,57,23,90,68,'#327b9c')
for x in [50,100,150]: box(x,195,30,38,18,'#3483a6')
label(390,90,'01 / RECYCLING')
parts.append('</g><g data-facility="1">')
box(320,55,140,120,50,'#215b80')
for x in [335,380,425]: box(x,190,30,50,22,'#2c739b')
box(300,300,150,40,8,'#1a4667')
label(690,220,'02 / COLLECTION')
parts.append('</g><g data-facility="2">')
for y in [315,355,395]: box(35,y,180,23,16,'#236b7f')
box(120,445,95,40,24,'#287e9d')
box(300,390,145,75,40,'#245874')
label(130,350,'03 / COMPOST + MULCH')
parts.append('</g>')
for x,y in [(235,30),(235,285),(480,270),(25,265)]:
    px,py=iso(x,y)
    parts.append(f'<circle cx="{px}" cy="{py}" r="4" fill="#90e3ff"/>')
label(350,505,'SEPARATE / PROCESS / QUALIFY / RETURN','#6da9c8',10)
parts.append('</svg>')
(root/'public/recovery-campus.svg').write_text(''.join(parts),encoding='utf-8')
