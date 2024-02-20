def Settings( **kwargs ):
  if kwargs[ 'language' ] == 'rust':
    return {
      'ls': {"checkOnSave": False, "diagnostics": {"enable": False}}
      #  {"cachePriming": { "enable": False }, "check": { "allTargets": False }, "checkOnSave": False, "diagnostics": {"enable": False} }
    }
