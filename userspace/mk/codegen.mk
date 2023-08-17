USER_SCRIPTS = $(USER_PATH)/scripts
USER_GENERATED = $(USER_PATH)/generated

VPATH += $(USER_GENERATED)

# make sure folder exists and is empty
$(shell mkdir -p $(USER_GENERATED))
$(shell rm -rf $(USER_GENERATED)/*)

# enabled_features_t
$(shell $(USER_SCRIPTS)/features.py $(USER_GENERATED))
SRC += generated_features.c \
       generated_features_draw.c

# QP assets
ifeq ($(strip $(QUANTUM_PAINTER_ENABLE)), yes)
    QP_DIRS := $(KEYBOARD_PATHS) $(KEYMAP_PATH) $(USER_PATH)
    $(shell $(USER_SCRIPTS)/qp_resources.py $(USER_GENERATED) $(QP_DIRS))
    SRC += generated_qp_resources.c
    include $(USER_GENERATED)/generated_qp_resources.mk
endif
