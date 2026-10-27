"""Generate the original, code-native isometric architectural illustration."""
from pathlib import Path
from math import sin,cos,pi
import json

ROOT=Path(__file__).parent
parts=[]
def p(x,y,z=0): return (560+(x-y)*0.83,94+(x+y)*0.44-z)
def points(seq): return ' '.join(f'{x:.1f},{y:.1f}' for x,y in seq)
def poly(seq,fill,stroke='#277b9e',width=.7,extra=''):
    parts.append(f'<polygon points="{points(seq)}" fill="{fill}" stroke="{stroke}" stroke-width="{width}" {extra}/>')
def line(a,b,color='#349ec7',width=1,extra=''):
    parts.append(f'<path d="M{a[0]:.1f},{a[1]:.1f} L{b[0]:.1f},{b[1]:.1f}" fill="none" stroke="{color}" stroke-width="{width}" {extra}/>')
def slab(x,y,w,d,z=0,fill='#0d2232'):
    poly([p(x,y,z),p(x+w,y,z),p(x+w,y+d,z),p(x,y+d,z)],fill)
def box(x,y,w,d,h,z=0,roof='#163849'):
    poly([p(x,y+d,z),p(x+w,y+d,z),p(x+w,y+d,z+h),p(x,y+d,z+h)],'#0c2336','#286480')
    poly([p(x+w,y,z),p(x+w,y+d,z),p(x+w,y+d,z+h),p(x+w,y,z+h)],'#071827','#205879')
    slab(x,y,w,d,z+h,roof)
    # Glass frontage and precise illuminated floor lines.
    for zz in range(9,int(h)-2,13):
        line(p(x+4,y+d,z+zz),p(x+w-4,y+d,z+zz),'#1b6d96',.75)
        line(p(x+w,y+4,z+zz),p(x+w,y+d-4,z+zz),'#165679',.6)
    for xx in range(7,int(w)-3,13):
        line(p(x+xx,y+d,z+5),p(x+xx,y+d,z+h-5),'#245771',.6)
    line(p(x,y,z+h),p(x+w,y,z+h),'#66daff',1)
    line(p(x+w,y,z+h),p(x+w,y+d,z+h),'#3da9d9',1)
def cylinder(x,y,r,h):
    top=[p(x+cos(a*pi/16)*r,y+sin(a*pi/16)*r,h) for a in range(33)]
    bottom=[p(x+cos(a*pi/16)*r,y+sin(a*pi/16)*r) for a in range(17)]
    poly(bottom+list(reversed(top[:17])),'#102b40','#2c7598')
    poly(top,'#1b4056','#56c7e8')
    line(p(x+r,y,h),p(x+r,y),'#4993b6')
def tree(x,y):
    a=p(x,y);line(a,p(x,y,13),'#2b8398',1.5)
    q=p(x,y,21)
    poly([(q[0],q[1]-8),(q[0]+7,q[1]+6),(q[0],q[1]+11),(q[0]-7,q[1]+6)],'#123d4b','#297e8b')

parts.append('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1120 730"><defs><radialGradient id="halo"><stop stop-color="#065395" stop-opacity=".35"/><stop offset="1" stop-color="#030b16" stop-opacity="0"/></radialGradient><linearGradient id="base" x2="1" y2="1"><stop stop-color="#12344e"/><stop offset="1" stop-color="#07111f"/></linearGradient><filter id="glow"><feGaussianBlur stdDeviation="3"/></filter></defs>')
parts.append('<ellipse cx="560" cy="445" rx="520" ry="280" fill="url(#halo)"/>')
for q in range(-80,701,40):
    line(p(q,-30),p(q,630),'#103247',.5)
    line(p(-30,q),p(630,q),'#103247',.5)
# Floating plinth with a luminous perimeter.
poly([p(-12,-12,-18),p(612,-12,-18),p(612,612,-18),p(-12,612,-18)],'url(#base)','#123d5f')
poly([p(-12,612),p(612,612),p(612,612,-18),p(-12,612,-18)],'#071524','#1c4465')
poly([p(612,-12),p(612,612),p(612,612,-18),p(612,-12,-18)],'#040e1b','#1c4465')
slab(-12,-12,624,624,0,'#091924')
for q in (193,395):
    slab(q,-5,24,610,1,'#06101b');slab(-5,q,610,24,1,'#06101b')
    for n in range(0,600,30):
        line(p(q+12,n,2),p(q+12,n+14,2),'#49bded',1)
        line(p(n,q+12,2),p(n+14,q+12,2),'#49bded',1)
# Distinct district platforms.
for x in (5,220,422):
    for y in (5,220,422): slab(x,y,173,173,2,'#0c2330')
# Agriculture: greenhouse roofs and ordered growing beds.
for x in (20,75,130):
    for y in (20,55,90):
        slab(x,y,40,23,3,'#194c48')
        for yy in range(y+3,y+23,5):line(p(x+2,yy,4),p(x+38,yy,4),'#419e9f',1)
for x in (25,83):
    box(x,128,48,35,14,roof='#194451')
    poly([p(x,128,14),p(x+24,128,34),p(x+48,128,14),p(x+48,163,14),p(x+24,163,34),p(x,163,14)],'#20515c','#65bdd1')
    for xx in range(8,48,8): line(p(x+xx,128,14+min(xx,48-xx)*.83),p(x+xx,163,14+min(xx,48-xx)*.83),'#72c6d4',.65)
# Energy: solar fields, turbines, storage.
for x in (238,275,312):
    for y in (20,52,84):
        box(x,y,29,22,5,roof='#0e426a')
        for k in (8,17):line(p(x+k,y,6),p(x+k,y+22,6),'#3da3d1',.7)
for x,y in ((253,145),(340,143)):
    line(p(x,y),p(x,y,104),'#88d9f5',3)
    a=p(x,y,104)
    for ang in (0,2*pi/3,4*pi/3):line(a,(a[0]+sin(ang)*34,a[1]-cos(ang)*34),'#b2e6fc',3)
box(355,30,24,86,23,roof='#24617d')
# Water: tanks and treatment basins.
for x,y in ((450,35),(510,35),(450,97)):
    cylinder(x,y,20,24)
slab(493,80,75,67,6,'#0a6683')
for k in (0,14,28,42):line(p(500,87+k,7),p(560,87+k,7),'#54b7d2',1)
box(432,143,128,27,20)
# Food and grocery.
box(20,238,135,68,37,roof='#19506a')
box(33,250,100,42,41,roof='#163a4e')
for x in (23,63,103):box(x,331,27,40,12,roof='#297191')
for y in (318,334,350):line(p(30,y,1),p(160,y,1),'#153f59')
# Center headquarters: multi-height crystalline structures.
box(244,240,51,60,90,roof='#2b617e')
box(303,254,57,72,136,roof='#2b7093')
box(269,320,80,42,60,roof='#16435e')
line(p(330,285,136),p(330,285,177),'#5bd6ff',1.3)
q=p(330,285,184)
poly([(q[0],q[1]-11),(q[0]+10,q[1]),(q[0],q[1]+11),(q[0]-10,q[1])],'#3ac7ff','#b1edff')
# Medical campus with cross.
box(447,245,105,65,50,roof='#18516b')
box(476,228,47,100,51,roof='#1c5d79')
slab(483,254,31,9,53,'#8feaff');slab(494,243,9,31,53,'#8feaff')
box(442,341,115,31,23,roof='#173e56')
# Homes and neighborhood.
for x,y,h in ((25,440,66),(82,440,92),(139,440,51),(32,514,46),(95,520,62)):
    box(x,y,36,46,h,roof='#225370')
    slab(x+5,y+6,26,32,h+1,'#316f80')
for x,y in ((8,498),(65,492),(145,518),(156,567),(67,573)):tree(x,y)
# Industrial hub.
box(237,445,116,78,35,roof='#1d4d67')
for x in (248,280,312):box(x,451,22,65,44,roof='#2a6580')
for x in (240,279):cylinder(x,557,15,29)
box(327,542,34,35,30)
line(p(257,527,6),p(257,550,6),'#6bc7e9',4)
# Fuel / logistics.
box(443,437,112,55,30,roof='#204e66')
for y in (509,542):
    box(444,y,67,15,8,roof='#236887')
    box(512,y,18,15,11,roof='#4ba2c3')
for x in (548,576):cylinder(x,556,11,27)
for x in (450,478,505):box(x,582,7,7,25,roof='#8ae1f7')
slab(442,574,84,24,27,'#1f7da9')
# Streetscape and active data paths.
for x,y in ((177,50),(180,116),(228,204),(393,235),(385,370),(424,390),(590,278),(205,560),(392,500),(88,390)):tree(x,y)
for a,b in [((205,100),(205,505)),((100,407),(500,407)),((405,100),(405,560))]:
    line(p(*a,3),p(*b,3),'#2cbef2',2,'stroke-dasharray="4 12" opacity=".8"')
parts.append('</svg>')
(ROOT/'public/atlas.svg').write_text(''.join(parts),encoding='utf-8')
print('Generated public/atlas.svg')
