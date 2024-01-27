

# Godot commands

Run project in editor:

```
godot godot/project.godot
cargo build ; godot godot/project.godot
```

Start scene in debug mode without editor (must be run from godot folder):

```
godot -d node_2d.tscn
```

# Deployment

ssh -i "./.ssh/cta-udp-real-time.pem" ubuntu@35.181.43.91

# Game Design

## Chore principles

- Online multiplayer
- 2D Isometric
- Move with mouse click
- Pathfinding
- Damage sources: Attack | Spell | DoT 
- Damage types: Physical | Fire | Cold | Lightning | Chaos
- Defence types: Armour | Physical damage reduction | Block | Evasion | Resistance | Less damage taken (all of specifics sources/types)
- Weapon types: Sword(1h|2h) | Axe(1h|2h) | Staff(2h) | Wand(1h) | Bow(2h) | Shield(1h)
- Speed: Action speed (all) | Attack speed | Casting speed | Movement speed
- Values calculation: increase | reduced (additive) vs more | less (multiplicative)
- No class (not when you first create your character at least, you build your character toward an archetype of your choice)
- Skill tag modifier: AOE (+/-) PROJ (+/- split fork bounce) ...

ailments (damaging / non damaging (include buff/debuff))

## Skill Building (one tree per skill)

1 - spell attack
2 - projectile melee
3 - aoe (shape: distance + angle, each weapons have its base value) target

4 - (not unlocked by default) optional secondary effect (with condition: on hit | on reach max range) loop back to 2

stats apply to primary of secondary

## Passives Tree

Add filler passives to reach powerfull ones or split effect (even none splitable ones, but on threhold can be reached from 
other sources eg: +0.5 projectile)

Passives points are capped (max level)

Modifiers to one (or multiple) of each skills elements with down sides (from 2 to 3)
Add bonus to skill element but downside to the its counterpart
offence:
    projectile:
        number 
        bounce
        chain
    melee:
        range
    aoe:
        size
        distance
        angle
    target:
        number
    ailments:
        self:
            damaging:
                blood rage (passive or give skill)
            non damaging:
        enemy:
            damaging:
                % chance to ignite with melee fire skills
                % chance to ignite with prjectile fire skills
            non damaging:
defence:
    element:
        reduce damage
    ailment:
        reduce damage
        chance to avoid
    defence:
        big key stones
    regen:
        on hit
        leach
        MoM

## Game Loop

Struggle to kill a few mobs
Rework you skill tree based on found items
Loot items
Equip or gamble items in some way to maybe get an upgrade

Random events
Deterministic events (counter: eg: every X maps)
Pseudo determinitict events (counter: eg: loot N shard to create a key to open a special map)
Unique fragments








