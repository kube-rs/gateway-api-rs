## Type Reduction 

This application will parse Kopium generated files and will try to identify the types that are potentially the same. The new types will be saved into "common" mod with a new, user selected name and the code will be updated with the new names.
The overall approach has three steps.

### 1. Reducing leaf types.
The algorithm will try to identify the structs that can be reduced or "leaf" types. Leaf types are the types with fields which are simple types (String, u32, u64) or types reduced in the previous steps. As the output, the application will produce files with "mappings". 

### 2. Provide new names
The mappings from step 1 should be used to provide new, user selected names.


##### Before the change. 
This shows that all above Kopium generated types are the same and we should replace "GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd" with a more meaningful name.

| Kopium generated names | |  User selected name| 
|------------------------|--|-------------------|
|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierSet|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|GRPCRouteRulesBackendRefsFiltersResponseHeaderModifierAdd|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|GRPCRouteRulesBackendRefsFiltersResponseHeaderModifierSet|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|HTTPRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|HTTPRouteRulesBackendRefsFiltersRequestHeaderModifierSet|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|HTTPRouteRulesBackendRefsFiltersResponseHeaderModifierAdd|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|
|HTTPRouteRulesBackendRefsFiltersResponseHeaderModifierSet|->|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|


##### After the change. 
On subsequent runs, the algorithm will use HTTPHeader as new name for all those types.


| Kopium generated names | |  User selected name| 
|------------------------|--|-------------------|
|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|->|HTTPHeader|
|GRPCRouteRulesBackendRefsFiltersRequestHeaderModifierSet|->|HTTPHeader|
|GRPCRouteRulesBackendRefsFiltersResponseHeaderModifierAdd|->|HTTPHeader|
|GRPCRouteRulesBackendRefsFiltersResponseHeaderModifierSet|->|HTTPHeader|
|HTTPRouteRulesBackendRefsFiltersRequestHeaderModifierAdd|->|HTTPHeader|
|HTTPRouteRulesBackendRefsFiltersRequestHeaderModifierSet|->|HTTPHeader|
|HTTPRouteRulesBackendRefsFiltersResponseHeaderModifierAdd|->|HTTPHeader|
|HTTPRouteRulesBackendRefsFiltersResponseHeaderModifierSet|->|HTTPHeader|


### 3. Re-run the application to produce the code with desired types


Steps 1 to 3 should be repeated until no similar types are detected. Check [update.sh](../update.sh) for more details on how to use it.

