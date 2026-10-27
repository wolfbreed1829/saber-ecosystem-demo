"""Original vector concept models for the family-life blueprint; not scaled designs."""
from pathlib import Path
from math import sin, cos, pi
import generate_atlas as a

ROOT = Path(__file__).parent

def start(title):
    a.parts.clear()
    a.parts.append(f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1120 730" role="img"><title>{title}</title><defs><radialGradient id="glow"><stop stop-color="#127ab4" stop-opacity=".3"/><stop offset="1" stop-color="#030b16" stop-opacity="0"/></radialGradient></defs><ellipse cx="560" cy="420" rx="520" ry="290" fill="url(#glow)"/>')
    for q in range(-40,661,40):
        a.line(a.p(q,-20),a.p(q,620),'#123249',.45)
        a.line(a.p(-20,q),a.p(620,q),'#123249',.45)
    a.poly([a.p(0,0),a.p(600,0),a.p(600,600),a.p(0,600)],'#0a1c2b','#2f749c')
    a.poly([a.p(0,600),a.p(600,600),a.p(600,600,-16),a.p(0,600,-16)],'#071523','#244c69')
    a.poly([a.p(600,0),a.p(600,600),a.p(600,600,-16),a.p(600,0,-16)],'#05101c','#244c69')

def finish(name):
    a.parts.append('</svg>')
    (ROOT/'public'/name).write_text(''.join(a.parts),encoding='utf-8')
    print('Generated',name)

def person(x,y,color='#99dfff'):
    xx,yy=a.p(x,y,6)
    a.parts.append(f'<circle cx="{xx}" cy="{yy-8}" r="2" fill="{color}"/>')
    a.line((xx,yy-5),(xx,yy+1),color,2)
    a.line((xx,yy),(xx-2,yy+4),color,1)
    a.line((xx,yy),(xx+2,yy+4),color,1)

def bench(x,y):
    a.box(x,y,20,7,5,roof='#44738a')
    a.box(x,y,20,2,11,roof='#75b6d2')

def roof(x,y,w,d,h):
    a.box(x,y,w,d,h,roof='#194762')
    a.poly([a.p(x,y,h),a.p(x+w/2,y,h+14),a.p(x+w,y,h),a.p(x+w,y+d,h),a.p(x+w/2,y+d,h+14),a.p(x,y+d,h)],'#225570','#6bcaf0')

def swing(x,y,size=1):
    for xx in (x,x+55*size):
        a.line(a.p(xx,y),a.p(xx,y+12*size,37*size),'#9bdcfa',2)
        a.line(a.p(xx,y+24*size),a.p(xx,y+12*size,37*size),'#9bdcfa',2)
    a.line(a.p(x,y+12*size,37*size),a.p(x+55*size,y+12*size,37*size),'#a6e6ff',3)
    for xx in (x+13*size,x+36*size):
        for dx in (0,9*size):a.line(a.p(xx+dx,y+12*size,35*size),a.p(xx+dx,y+12*size,8*size),'#66b5d7',1)
        a.slab(xx,y+9*size,9*size,7*size,8*size,'#61ceff')

def slide(x,y,size=1):
    a.box(x,y,25*size,25*size,30*size,roof='#3a87ae')
    a.poly([a.p(x+25*size,y+6*size,30*size),a.p(x+65*size,y+6*size,2),a.p(x+65*size,y+19*size,2),a.p(x+25*size,y+19*size,30*size)],'#2a89b8','#8ce2ff')
    a.line(a.p(x,y,50*size),a.p(x+25*size,y,50*size),'#4ecaff',2)
    for xx in (x,x+25*size):a.line(a.p(xx,y,30*size),a.p(xx,y,50*size),'#a4dbef',1.4)
    a.poly([a.p(x-3*size,y-3*size,50*size),a.p(x+12*size,y+12*size,65*size),a.p(x+28*size,y-3*size,50*size),a.p(x+28*size,y+28*size,50*size),a.p(x+12*size,y+12*size,65*size),a.p(x-3*size,y+28*size,50*size)],'#225673','#4bbde9')

def court(x,y,w,d):
    a.slab(x,y,w,d,3,'#144057')
    for o in (5,):a.poly([a.p(x+o,y+o,4),a.p(x+w-o,y+o,4),a.p(x+w-o,y+d-o,4),a.p(x+o,y+d-o,4)],'none','#65adc8',.8)
    a.line(a.p(x+w/2,y+4,4),a.p(x+w/2,y+d-4,4),'#65adc8',.8)
    ring=[a.p(x+w/2+cos(t*pi/16)*13,y+d/2+sin(t*pi/16)*13,4) for t in range(33)]
    a.poly(ring,'none','#65adc8',.8)

# Neighborhood: a human-scale extension of the original nine-district atlas.
start('Saber Tech family neighborhood concept')
for q in (190,390):
    a.slab(q,0,24,600,1,'#10283a');a.slab(0,q,600,24,1,'#10283a')
    for n in range(15,590,35):
        a.line(a.p(q+12,n,2),a.p(q+12,n+13,2),'#4db7dd',.8)
        a.line(a.p(n,q+12,2),a.p(n+13,q+12,2),'#4db7dd',.8)
# Childcare and protected garden.
roof(25,35,120,65,25)
a.slab(30,113,115,52,2,'#18444e')
slide(40,124,.48)
for x,y in ((20,130),(150,80),(150,150)):a.tree(x,y)
# School around a courtyard, visibly different from the office tower.
a.box(235,25,130,43,48,roof='#246489')
a.box(235,68,37,95,28,roof='#215578');a.box(330,68,35,95,28,roof='#215578')
a.slab(278,80,44,74,2,'#194553')
for y in (95,125):bench(288,y)
for x in (245,266,287,308,329):a.slab(x,30,15,28,49,'#267da8')
# Library and maker wing.
a.box(435,35,125,65,43,roof='#285b79');a.box(455,102,88,52,24,roof='#1a4e6b')
for x in range(443,555,13):a.line(a.p(x,100,6),a.p(x,100,37),'#5db8df',1)
# Family housing with gardens.
for x,y in ((25,242),(94,242),(25,315),(94,315)):roof(x,y,45,45,32)
for x,y in ((80,240),(150,298),(80,350),(20,300)):a.tree(x,y)
# Central park: play is actual equipment, not a generic block.
a.slab(225,225,150,150,2,'#123e46')
a.slab(244,243,112,72,3,'#205569')
slide(255,251,.8);swing(295,327,.8)
for x,y in ((233,235),(365,240),(232,345),(362,360)):a.tree(x,y)
bench(242,350);bench(334,236)
# Family care center.
a.box(435,240,120,72,35,roof='#285f7e')
a.slab(480,260,30,8,37,'#b0eaff');a.slab(491,249,8,30,37,'#b0eaff')
a.box(450,329,90,35,20,roof='#1c415c')
# Sports and pool.
court(20,430,86,132)
a.box(123,435,48,120,8,roof='#37a8ca')
for x in (132,141,150,159):a.line(a.p(x,445,10),a.p(x,545,10),'#b2eaff',.8)
# Grocery, community hall, and public square.
a.box(235,430,65,55,35,roof='#266387');a.box(310,430,63,55,35,roof='#326586')
for x in (240,270,300,330):
    for xx in (x,x+18):a.line(a.p(xx,518),a.p(xx,518,18),'#6da1ba',1)
    a.slab(x,507,18,25,19,'#449dc3')
for x,y in ((250,560),(335,555)):bench(x,y)
# Elder residence and community gardens.
a.box(440,432,110,55,34,roof='#284e6a')
for x in (445,478,511):
    for y in (520,548):a.box(x,y,24,15,7,roof='#297f83')
for x,y in ((420,560),(565,510),(567,450)):a.tree(x,y)
for x,y in ((200,260),(200,350),(290,200),(320,400),(405,300),(470,400),(280,545),(345,292),(140,210),(210,495)):person(x,y)
finish('neighborhood.svg')

# Roof-open school campus diagram.
start('Saber school campus with learning spaces and outdoor court')
rooms=[(35,35,150,130,'#16435c'),(220,35,150,130,'#164f68'),(405,35,150,130,'#174158'),(35,220,150,140,'#183e59'),(405,220,150,140,'#1b4057'),(35,405,150,140,'#173b52'),(220,405,150,140,'#13485a'),(405,405,150,140,'#204e62')]
for x,y,w,d,color in rooms:
    a.slab(x,y,w,d,3,color)
    a.box(x,y,w,4,25,roof='#438bb0');a.box(x,y,4,d,25,roof='#438bb0')
    a.line(a.p(x+w,y,5),a.p(x+w,y+d,5),'#5db3d8',1)
    a.line(a.p(x,y+d,5),a.p(x+w,y+d,5),'#5db3d8',1)
# Classroom desks.
for x in (60,102,144):
    for y in (74,110,144):a.box(x,y,21,14,12,roof='#35769b')
a.box(55,42,110,6,29,roof='#0c233c')
# Science benches and maker machinery.
for x in (245,290,335):a.box(x,70,25,72,14,roof='#45859b')
for x,y in ((430,67),(485,67),(430,120),(485,120)):a.box(x,y,40,22,16,roof='#2a6f9c')
# Library shelving and reading seats.
for x in (55,92,129):a.box(x,235,14,65,30,roof='#29607c')
for x in (65,122):a.box(x,320,30,20,9,roof='#357b97')
# Courtyard with trees and visible routes.
a.slab(220,220,150,140,2,'#163f47')
for x,y in ((240,245),(340,245),(240,340),(340,340)):a.tree(x,y)
bench(272,265);bench(272,315)
for x in range(190,400,20):a.line(a.p(x,190,3),a.p(x+10,190,3),'#60c4e9',1.5)
# Dining tables and kitchen.
a.box(425,235,110,22,22,roof='#2d718b')
for x in (435,490):
    for y in (285,325):a.box(x,y,36,18,11,roof='#5594a9')
# Support spaces, outdoor learning, and arrival.
for x,y in ((52,427),(113,427),(52,487),(113,487)):a.box(x,y,40,32,12,roof='#2e617e')
for x in (240,282,324):a.box(x,430,25,28,7,roof='#2e7b7e')
for x,y in ((242,500),(325,500)):bench(x,y)
a.slab(415,421,130,60,5,'#204b62')
for x in (430,470,510):a.box(x,490,21,13,10,roof='#5389a6')
for x,y in ((205,200),(390,200),(205,405),(390,405),(290,285),(470,525)):person(x,y)
finish('school-campus.svg')

# Playground concept: differentiated activities and inclusive shared paths.
start('Saber park and playground concept with five activity zones')
a.slab(20,20,560,560,2,'#10333f')
for q in (190,390):a.slab(q,20,22,560,3,'#225267');a.slab(20,q,560,22,3,'#225267')
# Early play.
a.slab(40,40,130,130,4,'#24556a');slide(65,70,.65)
for x,y in ((60,140),(135,140)):a.cylinder(x,y,12,7)
# Large adventure structures and connection bridge.
slide(235,55,1.25);slide(330,125,.7)
a.slab(265,70,65,14,37,'#3c9ab7')
for x in (267,285,303,325):a.line(a.p(x,70,38),a.p(x,70,54),'#9ce1ef',1)
a.line(a.p(267,70,54),a.p(325,70,54),'#81d3ed',1.5)
# Shared inclusive play pavilion with ramp.
a.slab(445,50,100,100,22,'#2c738f')
for x,y in ((445,50),(545,50),(445,150),(545,150)):a.line(a.p(x,y),a.p(x,y,55),'#75bcd8',2)
a.slab(435,40,120,120,56,'#285a73')
a.poly([a.p(445,150,22),a.p(475,150,22),a.p(475,183,3),a.p(445,183,3)],'#52a2bb','#a6e2f6')
# Sports.
court(35,240,130,125)
for x in (40,160):
    a.line(a.p(x,300),a.p(x,300,42),'#97ddf4',2)
    a.poly([a.p(x,288,42),a.p(x,312,42),a.p(x,312,57),a.p(x,288,57)],'#245276','#93c8e4')
# Central social lawn.
a.slab(230,230,140,140,4,'#184149')
swing(260,280,1.4)
# Quiet sensory garden.
for x,y in ((445,245),(510,245),(450,320),(530,330)):a.tree(x,y)
for x in (450,495,540):a.box(x,275,22,18,8,roof='#388988')
bench(470,340)
# Facilities, shaded caregivers' seats, and open lawn.
a.box(40,450,115,67,27,roof='#235576')
for x in (230,300):
    a.slab(x,460,40,50,36,'#377895')
    for xx in (x,x+40):a.line(a.p(xx,460),a.p(xx,460,36),'#91cbe2',1.5)
    bench(x+8,470)
for x,y in ((448,455),(525,460),(470,545),(540,550)):a.tree(x,y)
for x,y in ((65,205),(200,80),(400,180),(200,460),(400,420),(360,560),(100,560),(560,100),(560,375)):a.tree(x,y)
for x,y in ((205,260),(205,350),(285,200),(405,290),(335,405),(285,530)):person(x,y)
finish('playground.svg')
