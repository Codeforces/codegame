import model


class MyStrategy:
    def get_action(self, player_view, debug):
        raise NotImplementedError()

    def debug_update(self, debug):
        pass
