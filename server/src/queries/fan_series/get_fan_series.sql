SELECT fan_sizes.id as fan_size_id, diameter, fan_serieses.id as fan_series_id, fan_type
            FROM fan_serieses
            LEFT JOIN fan_sizes
            ON fan_sizes.fan_series_id = fan_serieses.id
            WHERE fan_serieses.id = $1