require_relative 'model'

class MyStrategy
    def get_action(player_view, debug)
        raise NotImplementedError
    end
    def debug_update(debug)
    end
end