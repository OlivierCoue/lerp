Initial Effect:
    Projectile | Strike | AOE (relativity (Player | Direction | Cursor))

Additional Effect:
    Projectile | AOE (relativity (previous skill effect))

Additional Effect Trigger Condition:
    Projectile:
        On hit
        On max range reached
        On tick (every x millis, eg: Frozen Orb)    
    Strike:
        On hit
        On attack (can trigger effect while hitting nothing)
    AOE:
        On hit
        On duration end

AOE is composed of:
    Shape:
        Circle
        Rectangle (add angle from origin)
        Cone (range + angle) starting from origin location and pointing to cursor if origin is the player, else, point to the nearest enemy (may be broken)
    Size:
    Origin location (can only be mannualy chosed when AOE is the initial effect):
        Player
        Cursor (add max Range)
        Previous skill effect 
    Location from origin:
        At Origin
        Nearest enemy (add another type of max Range)
    Duration (optional, enable DOT, Slow...):
    Arrangement around origin (only apply when their is more than one AOE):
        Circle
        Line
        Cross

Projectile is composed of:
    Range
    Chain count
    Pierce count
    Split count
    Direction
        Random
        Homing
        Evenly spaced around origin
        Cursor

Strike is composed of:
    Range

Itemised Skill Effect with socket slot
    Inital Skill effect:
    1 Initial Attack Lighning Strike -> Socket slot
    2 Initial Attack Chaos Projectile -> Socket slot

    Additional Skill effect (inherit initial effect damage type)
    3 Additional Attack Projetile (Have 5 base projectiles, fire in the same direction as player, deal 50 less damage) -> Socket slot
    4 Additional Attack AOE (Circle Shape, Duration of 2s, DOT) -> Socket slot

    1 + 3 = Lighting Strike
    2 + 4 = Caustic Arrow
    1 + 4 = Attaque au cac qui pose des AOE Lighning qui dot, new skill ?
    2 + 3 = Genre de Lightning Strike a distance qui fais des Chaos damage, new skill ?

En gros les skill effect tu les buy ou les trouve un peux comme des cluster jewel, et on pourrais avec avoir des effect avec 2 slot qui créé deux effect
Genre projectile qui on hit fais une AOE + envois d'autre projectile
Et ça permet de balance car les on prédéfini les stats des skill effect qui existe, ça revient au gems de POE sauf que tu split le truc encore plus, tu as pas un
skill que tu vient juste twick les valeurs (avec les support), tu définit ses mecanique en partant d'un base skill très basique