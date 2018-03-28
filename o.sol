contract my_bux {
  //Permissions and empowerments.
  //Exogenous events.
  bool create_contract_perm_wildcard_on_0 = false;
  bool create_contract_pow_wildcard_on_0 = false;
  mapping (address => boolean) private create_contract_perm_map_on_0;
  mapping (address => boolean) private create_contract_pow_map_on_0;
  bool transfer_perm_wildcard_on_0 = false;
  bool transfer_pow_wildcard_on_0 = false;
  mapping (address => boolean) private transfer_perm_map_on_0;
  mapping (address => boolean) private transfer_pow_map_on_0;
  bool transfer_perm_wildcard_on_1 = false;
  bool transfer_pow_wildcard_on_1 = false;
  mapping (address => boolean) private transfer_perm_map_on_1;
  mapping (address => boolean) private transfer_pow_map_on_1;
  
  //Inst events.
  bool balance_down_perm_wildcard_on_0 = false;
  bool balance_down_pow_wildcard_on_0 = false;
  mapping (address => boolean) private balance_down_perm_map_on_0;
  mapping (address => boolean) private balance_down_pow_map_on_0;
  bool balance_up_perm_wildcard_on_0 = false;
  bool balance_up_pow_wildcard_on_0 = false;
  mapping (address => boolean) private balance_up_perm_map_on_0;
  mapping (address => boolean) private balance_up_pow_map_on_0;
  bool inst_creation_perm_wildcard_on_0 = false;
  bool inst_creation_pow_wildcard_on_0 = false;
  mapping (address => boolean) private inst_creation_perm_map_on_0;
  mapping (address => boolean) private inst_creation_pow_map_on_0;
  bool inst_transfer_perm_wildcard_on_0 = false;
  bool inst_transfer_pow_wildcard_on_0 = false;
  mapping (address => boolean) private inst_transfer_perm_map_on_0;
  mapping (address => boolean) private inst_transfer_pow_map_on_0;
  bool inst_transfer_perm_wildcard_on_1 = false;
  bool inst_transfer_pow_wildcard_on_1 = false;
  mapping (address => boolean) private inst_transfer_perm_map_on_1;
  mapping (address => boolean) private inst_transfer_pow_map_on_1;
  
  function my_bux {
    //Set initial wildcards
    inst_creation_perm_wildcard_on_0 = true;
    //Set initial wildcards
    create_contract_perm_wildcard_on_0 = true;
    //Set initial wildcards
    create_contract_pow_wildcard_on_0 = true;
    //Set initial wildcards
    balance_up_perm_wildcard_on_0 = true;
    //Set initial wildcards
    balance_down_perm_wildcard_on_0 = true;
  }
}
