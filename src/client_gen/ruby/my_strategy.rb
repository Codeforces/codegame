require_relative 'model'

class MyStrategy
    def get_action(player_view, debug_interface)
        raise NotImplementedError
    end
    def debug_update(player_view, debug_interface)
    end
end