import bpy
import math
from os.path import join

lerptools_ambient = (0.0212, 0.0212, 0.0212, 1)

def get_directions(n: int) -> list[float]:
    if n <= 0:
        raise ValueError("Parameter must be a positive integer.")
    
    step = 360.0 / n
    return [i * step for i in range(n)]

# Function to hide all collections
def hide_all_collections():
    for collection in bpy.data.collections:
        collection.hide_render = True
        collection.hide_viewport = True

# Function to set active collection
def set_active_collection(collection_name):
    hide_all_collections()
    if collection_name in bpy.data.collections:
        bpy.data.collections[collection_name].hide_render = False
        bpy.data.collections[collection_name].hide_viewport = False

##########
##  UI  ##
##########

class VIEW3D_PT_LerpToolsPanel(bpy.types.Panel):
    bl_label = "Lerp Tools"
    bl_idname = "VIEW3D_PT_LerpToolsPanel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = 'Lerp'
   
    def draw(self, context):
        layout = self.layout
        
        row = layout.row()
        row.prop(context.scene, "lerptools_types", expand = True)

class VIEW3D_PT_LerpToolsGenerate(bpy.types.Panel):
    bl_label = "Generate"
    bl_idname = "VIEW3D_PT_LerpToolsGenerate"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = 'Lerp'
    bl_parent_id = 'VIEW3D_PT_LerpToolsPanel'
   
    def draw(self, context):
        layout = self.layout
        
        row = layout.row()
        row.prop(context.scene, "lerptools_generate_default")
        
        row = layout.row()
        row.operator("lerptools.ops_generate")

        row = layout.row()
        row.operator("lerptools.ops_render")

#################
##  OPERATORS  ##
#################

class LERPTOOLS_OT_generate(bpy.types.Operator):
    bl_label = "Generate"
    bl_idname = "lerptools.ops_generate"
    bl_description = "Generate..."
    
    def scene_setup_common(self, context):
        # Set up output settings
        bpy.context.scene.render.engine = 'BLENDER_EEVEE_NEXT'
        
        # Denoise
        bpy.context.scene.cycles.use_denoising = True
        
        # Disable anti-alias
        bpy.context.scene.render.filter_size = 0.01
        
        # Black background
        bpy.context.scene.render.film_transparent = True
        bpy.data.worlds["World"].node_tree.nodes["Background"].inputs[0].default_value = lerptools_ambient
        
        # Maximum compression
        bpy.context.scene.render.image_settings.compression = 100

        return True

    def scene_setup_entity(self, context):
        self.scene_setup_common(context)
        
        # Output settings
        bpy.context.scene.render.resolution_x = 256
        bpy.context.scene.render.resolution_y = 256
        bpy.context.scene.frame_end = 8
        
        return True

    def execute(self, context):
        print("Running: LERPTOOLS_OT_generate")

        # Generate rotatebox
        rotatebox = bpy.data.objects.new('ROTATEBOX', None)
        bpy.context.collection.objects.link(rotatebox)
        rotatebox.empty_display_size = 0.5
        rotatebox.empty_display_type = 'PLAIN_AXES'
        
        # Generate camera
        cam_data = bpy.data.cameras.new('camera')
        cam = bpy.data.objects.new('CAMERA', cam_data)
        bpy.context.collection.objects.link(cam)
        bpy.context.scene.camera = cam
        cam.location = (0, -10, 8.5)
        cam.rotation_euler[0] = math.radians(60)
        cam.rotation_euler[1] = math.radians(0)
        cam.rotation_euler[2] = math.radians(0)
        cam.data.type = 'ORTHO'
        cam.data.ortho_scale = 7
        
        # Generate light source
        light_data = bpy.data.lights.new('light', type='SUN')
        light = bpy.data.objects.new('LIGHT', light_data)
        bpy.context.collection.objects.link(light)
        light.location = (8.05, -11.788, 24)
        light.rotation_euler[0] = math.radians(0)
        light.rotation_euler[1] = math.radians(30.7)
        light.rotation_euler[2] = math.radians(-55.7)
        
        # Parent camera and light to rotatebox, then rotate box to direction 0
        cam.parent = rotatebox
        light.parent = rotatebox
        rotatebox.rotation_euler[0] = math.radians(0)
        rotatebox.rotation_euler[1] = math.radians(0)
        rotatebox.rotation_euler[2] = math.radians(45)

        # Create root object for approximate scale example
        scales = bpy.data.objects.new('_Approximate_Scale_', None)
        bpy.context.collection.objects.link(scales)
        scales.empty_display_size = 0.5
        scales.empty_display_type = 'ARROWS'
        scales.hide_render = True
        
        # Example cubes representing approximately the height and stature of a human
        def new_bodypart(name, scale, location):
            bpy.ops.mesh.primitive_cube_add(
                size = 1,
                enter_editmode = False,
                align = 'WORLD',
                location = location,
                scale = scale,
            )
            bodypart = bpy.context.active_object
            bodypart.name = name
            bodypart.show_wire = True
            bodypart.display_type = 'WIRE'
            bodypart.parent = scales
            bodypart.hide_render = False
        
        # Generate cubes representing approximately the height and stature of a human
        new_bodypart('Human_Head', (0.2, 0.25, 0.25), (0, 0, 1.7))
        new_bodypart('Human_Torso', (0.5, 0.25, 0.7), (0, 0, 1.15))
        new_bodypart('Human_Leg_L', (0.15, 0.15, 0.8), (0.15, 0, 0.4))
        new_bodypart('Human_Leg_R', (0.15, 0.15, 0.8), (-0.15, 0, 0.4))
        
        self.scene_setup_entity(context)
        return {'FINISHED'}

class LERPTOOLS_OT_render(bpy.types.Operator):
    bl_label = "Render"
    bl_idname = "lerptools.ops_render"
    bl_description = "Render..."

    def execute(self, context):
        print("Running: LERPTOOLS_OT_render")

        rotatebox = bpy.data.objects["ROTATEBOX"]
        output_path = "//renders/"
        
        characters = ["archer", "enemy"]

        for character in characters:
            set_active_collection(character)

            # Replace 'Armature' with the name of your character's armature
            armature_name = "armature_" + character
            armature = bpy.data.objects.get(armature_name)

            animations = {
                "walk": {"name": "walk", "start_frame": 1, "frame_count": 16},
                "idle": {"name": "idle","start_frame": 1, "frame_count": 16},
                "attack": {"name": "attack","start_frame": 1, "frame_count": 16},
                "death": {"name": "death","start_frame": 1, "frame_count": 16},
                "dead": {"name": "death","start_frame": 16, "frame_count": 16},
            }
            
            for export_name, animation_data in animations.items():
                animation_name = animation_data["name"]
                start_frame = animation_data["start_frame"]
                frame_count = animation_data["frame_count"]
                armature.animation_data.action = bpy.data.actions[animation_name]

                for direction, angle in enumerate(get_directions(8)):
                    rotatebox.rotation_euler[2] = math.radians(angle) # Convert degrees to radians
                    bpy.context.view_layer.update()  # Update scene before rendering

                    direction_num = str( direction ).zfill(2) # Zero-padds frame number (1 -> 01)

                    # Render animation
                    for f in range(start_frame, frame_count + 1):
                        target_frame = f
                        bpy.context.scene.frame_set( target_frame ) # Set frame
                        
                        # Output definitions
                        
                        frame_num = str(target_frame - 1).zfill(4) # Zero-padds frame number (5 -> 0005)
                        frame_name = f"{character}/{export_name}/{direction_num}_{frame_num}{bpy.context.scene.render.file_extension}"
                        bpy.context.scene.render.filepath = join( output_path, frame_name )

                        # Render frame
                        bpy.ops.render.render(write_still = True)
                        print(f"Rendered {direction} {frame_num}")
            
        return {'FINISHED'}

################
##  REGISTER  ##
################

def enum_update_callback(self, context):
    directions = get_directions(8)
    
    rotatebox = bpy.data.objects["ROTATEBOX"]
    rotatebox.rotation_euler[2] = math.radians(directions[int(self.lerptools_types)])
    print(f"Enum changed to: {(360.0 / 8.0) * index}")

lerptools_types = bpy.props.EnumProperty(
    name = 'Directions',
    description = 'Directions',
    items = [
        ('0', 'S', 'S'),
        ('1', 'SW', 'SW'),
        ('2', 'W', 'W'),
        ('3', 'NW', 'NW'),
        ('4', 'N', 'N'),
        ('5', 'NE', 'NE'),
        ('6', 'E', 'E'),
        ('7', 'SE', 'SE'),
        
    ],
    default = '0',
    update = enum_update_callback
)

lerptools_generate_default = bpy.props.BoolProperty(
    name = "Generate Default",
    description = "Generate Default...",
    default = True,
)

registerClasses = [
    VIEW3D_PT_LerpToolsPanel,
    VIEW3D_PT_LerpToolsGenerate,
    LERPTOOLS_OT_generate,
    LERPTOOLS_OT_render,
]

def register():
    bpy.types.Scene.lerptools_types = lerptools_types
    bpy.types.Scene.lerptools_generate_default = lerptools_generate_default

    for c in registerClasses:
        bpy.utils.register_class(c)

def unregister():
    del bpy.types.Scene.lerptools_types
    del bpy.types.Scene.lerptools_generate_default

    for c in registerClasses:
        try:
            bpy.utils.unregister_class(c)
        except RuntimeError:
            print("Class was not registered before, skipping unregister")

if __name__ == "__main__":
    register()