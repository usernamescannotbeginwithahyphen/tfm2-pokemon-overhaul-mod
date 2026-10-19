"""Repack existing artwork into individual objects, phases and directional tags.

No new artwork is drawn here. Sources are retained so repeated runs are idempotent.
Run with Pillow. Generated Dedenne artwork is documented in vfx-repair-prompts.json.
"""
import json
import io
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'assets/custom_spritework'
VFX = ASSETS / 'vfx'
PACKAGE = ROOT / 'mod/pokemon_moba_stable'
BACKUP = ASSETS / 'references/vfx-pre-repair'
BACKUP.mkdir(parents=True, exist_ok=True)
NEAREST = Image.Resampling.NEAREST


def replace_bytes(path, data):
    if path.exists() and path.read_bytes() == data:
        return
    temp = path.with_name(path.name + '.pending')
    temp.write_bytes(data)
    # Windows preview/indexing can briefly hold a mapped image during regeneration.
    for attempt in range(10):
        try:
            temp.replace(path)
            return
        except PermissionError:
            if attempt == 9:
                raise
            time.sleep(0.2)


def frames(base, tag=None):
    data = json.loads(Path(str(base) + '#anim.fanim').read_text())
    sheet = Image.open(str(base) + '#sheet.png').convert('RGBA')
    anim = data['anims'][tag] if tag else next(iter(data['anims'].values()))
    result = []
    for frame in anim['frames']:
        p = frame['data']
        result.append((sheet.crop((p['x'], p['y'], p['x']+p['w'], p['y']+p['h'])), frame['duration']))
    return result


def original(stem):
    for suffix in ('#sheet.png', '#anim.fanim'):
        dest = BACKUP / (stem + suffix)
        if not dest.exists():
            shutil.copy2(VFX / (stem + suffix), dest)
    return frames(BACKUP / stem)


def pack(base, tags):
    size = max(max(im.size) for row in tags.values() for im, _ in row)
    cols = max(len(row) for row in tags.values())
    sheet = Image.new('RGBA', (size * cols, size * len(tags)))
    anims = {}
    for y, (tag, row) in enumerate(tags.items()):
        entries = []
        for x, (im, duration) in enumerate(row):
            sheet.alpha_composite(im, (x * size + (size-im.width)//2, y * size + (size-im.height)//2))
            entries.append(dict(duration=duration, data=dict(x=x*size, y=y*size, w=size, h=size)))
        anims[tag] = dict(frames=entries)
    base.parent.mkdir(parents=True, exist_ok=True)
    encoded = io.BytesIO()
    sheet.save(encoded, format='PNG')
    replace_bytes(Path(str(base)+'#sheet.png'), encoded.getvalue())
    Path(str(base) + '#anim.fanim').write_text(json.dumps(dict(anims=anims), indent=2)+'\n')


def centered(im, size=96, content=64):
    canvas = Image.new('RGBA', (size, size))
    if not im.getbbox():
        return canvas
    im = im.crop(im.getbbox())
    ratio = min(content/im.width, content/im.height)
    im = im.resize((max(1, round(im.width*ratio)), max(1, round(im.height*ratio))), NEAREST)
    canvas.alpha_composite(im, ((size-im.width)//2, (size-im.height)//2))
    return canvas


def crop_object(stem, box, content=56):
    row = original(stem)
    # One constant crop and scale preserves the authored object's pivot across frames.
    w, h = box[2]-box[0], box[3]-box[1]
    scale = content/max(w,h)
    result = []
    for im, duration in row:
        tile = im.crop(box).resize((round(w*scale), round(h*scale)), NEAREST)
        canvas = Image.new('RGBA', (96,96))
        canvas.alpha_composite(tile, ((96-tile.width)//2, (96-tile.height)//2))
        result.append((canvas,duration))
    pack(VFX/stem, {'effect':result})


def repair_objects():
    chain=original('dragalge_ult_toxic_chain_tether')
    links=[]
    for im,duration in chain[:5]:
        tile=im.crop((36,41,60,55)).resize((12,7),NEAREST)
        canvas=Image.new('RGBA',(32,32));canvas.alpha_composite(tile,(10,12))
        links.append((canvas,duration))
    pack(VFX/'dragalge_ult_toxic_chain_tether', {'chain':links})
    pack(VFX/'dragalge_ult_toxic_chain_impact', {'impact':chain[5:]})
    white=original('arcanine_skill2_white_flames_line_aoe')
    pack(VFX/'arcanine_skill2_white_flames_line_aoe',{'effect':[frame for i,frame in enumerate(white) if i!=4]})
    leaves=original('decidueye_basicattack_leafage_triple_arc')
    leaf=leaves[0][0].crop((55,24,83,39))
    canvas=Image.new('RGBA',(96,96));canvas.alpha_composite(leaf,((96-leaf.width)//2,(96-leaf.height)//2))
    pack(VFX/'decidueye_basicattack_leafage_projectile',{'flight':[(canvas,0.1)]})
    crop_object('clawitzer_skill1_water_sport_aoe', (29,0,67,53), 64)
    crop_object('dragalge_skill2_sludge_bomb_splashes', (34,0,61,60), 56)
    row = original('turtonator_skill1_shell_trap_field')
    tiles = [(centered(im.crop((52,0,109,62)), content=52),duration) for im,duration in row]
    pack(VFX/'turtonator_skill1_shell_trap_field', {'idle':tiles[:3]})
    pack(VFX/'turtonator_skill1_shell_trap_burst', {'burst':tiles[3:]})
    generated = ASSETS/'references/imagegen/dedenne_single_node_phases.png'
    im = Image.open(generated).convert('RGBA')
    assert im.getextrema()[3][0] == 0, 'Generated source must have real transparency'
    stems = ['nodes','chain','consume_heal','overload']
    for row, stem in enumerate(stems):
        tiles=[]
        for col in range(4):
            tile=im.crop((round(col*im.width/4),round(row*im.height/4),round((col+1)*im.width/4),round((row+1)*im.height/4)))
            # Fixed cell resize, not per-frame trim: no scale or pivot pumping.
            tile=tile.resize((96,96),NEAREST)
            tiles.append((tile,0.12 if row==0 else 0.08))
        pack(VFX/f'dedenne_ult_spark_grid_{stem}', {'effect':tiles})
    source=Image.open(ASSETS/'references/imagegen/charizard_effects_source.png').convert('RGBA')
    px=source.load()
    for y in range(source.height):
        for x in range(source.width):
            r,g,b,a=px[x,y]
            if g>100 and g>r*1.3 and g>b*1.3: px[x,y]=(0,0,0,0)
    tiles=[]
    for i in range(8):
        tile=source.crop((round(i*source.width/8),610,round((i+1)*source.width/8),source.height))
        tiles.append((centered(tile,content=76),0.075))
    pack(VFX/'charizard_ult_inferno_area',{'effect':tiles})
    pack(VFX/'blissey_skill1_heal_pulse_aura',{'effect':frames(ASSETS/'temp/blissey_skill1_heal_pulse_aura_bake')})
    bubble=original('octillery_skill2_bouncy_bubble_path')
    # A single bubble rides a vertical arc in sprite space while the engine moves its ground position.
    orb=bubble[0][0].crop((64,8,108,47))
    arc=[]
    for offset in [0,-8,-16,-22,-24,-22,-16,-8,0]:
        canvas=Image.new('RGBA',(128,128))
        canvas.alpha_composite(orb,((128-orb.width)//2,64-orb.height+offset))
        arc.append((canvas,12/60/9))
    pack(VFX/'octillery_skill2_bubble_flight',{'flight':arc})
    pack(VFX/'octillery_skill2_bubble_landing',{'land':[(centered(bubble[0][0].crop((40,45,135,96)),content=64),0.08)]})
    pack(VFX/'octillery_skill2_bouncy_bubble_path',{'burst':[(centered(bubble[i][0],content=72),0.08) for i in (4,5,8)]})
    zone=original('ampharos_ult_gigavolt_strike_zone')
    pack(VFX/'ampharos_ult_gigavolt_zone',{'zone':[(centered(zone[i][0],content=72),0.12) for i in (6,7)]})
    magma=original('coalossal_ult_magma_storm_line')
    pack(VFX/'coalossal_ult_magma_segment',{'segment':[(centered(magma[i][0].crop((0,0,96,38)),content=76),0.08) for i in (0,1,2,3)]})
    pack(VFX/'coalossal_ult_magma_burst',{'burst':[(centered(magma[6][0],content=72),0.12)]})
    for stem, box in [('dragapult_skill1_dragon_darts_projectile',(0,0,128,32)),
                      ('jolteon_basic_pin_missile_projectile',(0,0,96,22))]:
        row=original(stem)
        # Exclude the original combined impact frame; every flying object is now independent.
        tiles=[]
        for im,duration in row[:-1]:
            tile=im.crop(box)
            canvas=Image.new('RGBA',im.size)
            canvas.alpha_composite(tile,(0,(im.height-tile.height)//2))
            tiles.append((canvas,duration))
        pack(VFX/stem,{'flight':tiles})
    swanna=Image.open(ASSETS/'references/imagegen/swanna_sky_circus_phases.png').convert('RGBA')
    pixels=swanna.load()
    for y in range(swanna.height):
        for x in range(swanna.width):
            r,g,b,a=pixels[x,y]
            if g>100 and g>r*1.3 and g>b*1.3: pixels[x,y]=(0,0,0,0)
    for row,stem in enumerate(['swanna_ult_sky_circus_aoe','swanna_ult_sky_circus_slash','swanna_ult_sky_circus_landing']):
        cells=[]
        for col in range(4):
            tile=swanna.crop((round(col*swanna.width/4),round(row*swanna.height/3),round((col+1)*swanna.width/4),round((row+1)*swanna.height/3)))
            cells.append(tile)
        bounds=[tile.getbbox() for tile in cells]
        box=(min(b[0] for b in bounds),min(b[1] for b in bounds),max(b[2] for b in bounds),max(b[3] for b in bounds))
        scale=72/max(box[2]-box[0],box[3]-box[1])
        tiles=[]
        for tile in cells:
            tile=tile.crop(box).resize((round((box[2]-box[0])*scale),round((box[3]-box[1])*scale)),NEAREST)
            canvas=Image.new('RGBA',(96,96));canvas.alpha_composite(tile,((96-tile.width)//2,(96-tile.height)//2))
            tiles.append((canvas,0.1))
        pack(VFX/stem,{'effect':tiles})


def repair_bodies():
    text=(ROOT/'docs/custom-spritework-workflow.md').read_text(encoding='utf-8')
    tags={'charizard':['ult'],'banette':['skill','ult'],'rillaboom':['skill','skill2'],
          'armarouge':['skill'],'archaludon':['ult'],'blissey':['skill'],'thievul':['skill2'],'ceruledge':['skill']}
    for name, replace in tags.items():
        blocks=re.findall(r'```powershell\s*(.*?)```',text,re.S)
        command=next(b for b in blocks if f'champions\\{name} `' in b)
        args=re.findall(r'--[\w-]+|[^\s`]+',command.replace('`',' '))
        args=args[args.index('--species'):]
        args=[a.replace('\\','/') for a in args]
        out=ROOT/'scratch/vfx-clean-bodies'/name
        args[args.index('--output-base')+1]=str(out)
        subprocess.run([sys.executable,str(ROOT/'tools/convert-pmdcollab-sprite.py'),*args],cwd=ROOT,check=True,stdout=subprocess.DEVNULL)
        base=ASSETS/'champions'/name
        saved=BACKUP/'bodies'/name
        saved.parent.mkdir(exist_ok=True)
        for suffix in ('#sheet.png','#anim.fanim'):
            if not Path(str(saved)+suffix).exists(): shutil.copyfile(str(base)+suffix,str(saved)+suffix)
        old=json.loads(Path(str(saved)+'#anim.fanim').read_text())
        rows={tag:frames(out if tag in replace else saved,tag) for tag in old['anims']}
        if name=='archaludon':
            # Keep the original four bend/prep poses, then the clean channel body.
            rows['ult']=frames(saved,'ult')[:4]+rows['ult']
            total=sum(d for _,d in rows['ult'])
            rows['ult']=[(im,d*3/total) for im,d in rows['ult']]
        pack(base,rows)
        for suffix in ('#sheet.png','#anim.fanim'):
            dest=PACKAGE/'champions_custom'/(name+suffix)
            temp=dest.with_name(dest.name+'.pending')
            shutil.copyfile(str(base)+suffix,temp)
            temp.replace(dest)


def directional_tags():
    inventory=json.loads((ROOT/'scratch/vfx-runtime-inventory.json').read_text())
    policies={}
    for champion in inventory:
        for action in champion['actions']:
            effect=action['effect'].split(' {')[0]
            cone=effect.startswith('Cone')
            directional=cone or effect.startswith('Line') or effect in ('StarmiePsybeam','KecleonColorfulWhip')
            if directional:
                for asset in action['assets']:
                    if not asset['projectile']: policies[asset['stem']]='cone' if cone else 'center'
    for stem in ['goodra_skill1_rain_dance_field','turtonator_skill2_sunny_day_field',
                 'beeheeyem_skill2_light_screen','xatu_ult_super_psy_beam','yanmega_ult_giga_drain_tether',
                 'coalossal_ult_magma_segment','swanna_ult_sky_circus_slash',
                 'dragalge_ult_toxic_chain_tether']:
        policies[stem]='center'
    policies['arcanine_skill2_white_flames_line_aoe']='center'
    for stem,kind in policies.items():
        row=frames(VFX/stem) if stem in ('coalossal_ult_magma_segment','swanna_ult_sky_circus_slash','arcanine_skill2_white_flames_line_aoe','dragalge_ult_toxic_chain_tether') else original(stem)
        tags={'effect':row}
        for d in range(16):
            tiles=[]
            for im,duration in row:
                canvas=Image.new('RGBA',(192,192))
                if kind=='cone':
                    bounds=im.getbbox()
                    if bounds:
                        crop=im.crop(bounds)
                        scale=72/im.width
                        crop=crop.resize((max(1,round(crop.width*scale)),max(1,round(crop.height*scale))),NEAREST)
                        canvas.alpha_composite(crop,(102+round(bounds[0]*scale),96-crop.height//2))
                else:
                    canvas.alpha_composite(im,((192-im.width)//2,(192-im.height)//2))
                canvas=canvas.rotate(-d*22.5,resample=NEAREST)
                if kind=='cone':
                    lifted=Image.new('RGBA',(192,192)); lifted.alpha_composite(canvas,(0,-16)); canvas=lifted
                tiles.append((canvas,duration))
            tags[f'd{d}']=tiles
        pack(VFX/stem,tags)
    (ASSETS/'vfx-directional-policies.json').write_text(json.dumps(policies,indent=2)+'\n')


if __name__=='__main__':
    if '--sync-only' not in sys.argv:
        repair_objects()
        if '--assets-only' not in sys.argv: repair_bodies()
        directional_tags()
    for path in VFX.iterdir():
        if path.name.endswith(('#sheet.png','#anim.fanim')):
            dest=PACKAGE/'vfx'/path.name
            if not dest.exists() or path.read_bytes()!=dest.read_bytes():
                replace_bytes(dest, path.read_bytes())
    print('Repacked object/phase assets, selected clean body tags and directional variants.')
